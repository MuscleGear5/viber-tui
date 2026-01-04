use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use super::dispatch::AgentEvent;
use super::models::{AgentConfig, AgentId, AgentState};

pub struct AgentTask {
    pub id: AgentId,
    pub config: AgentConfig,
    pub state: AgentState,
}

pub enum PoolCommand {
    Spawn(AgentId, AgentConfig),
    Stop(AgentId),
    StopAll,
    Shutdown,
}

pub struct AgentPool {
    next_id: u64,
    command_tx: Sender<PoolCommand>,
    event_rx: Receiver<AgentEvent>,
    tasks: HashMap<AgentId, AgentTask>,
    worker_handle: Option<JoinHandle<()>>,
}

impl AgentPool {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel::<PoolCommand>();
        let (event_tx, event_rx) = mpsc::channel::<AgentEvent>();

        let worker_handle = thread::spawn(move || {
            Self::worker_loop(command_rx, event_tx);
        });

        Self {
            next_id: 1,
            command_tx,
            event_rx,
            tasks: HashMap::new(),
            worker_handle: Some(worker_handle),
        }
    }

    fn worker_loop(command_rx: Receiver<PoolCommand>, event_tx: Sender<AgentEvent>) {
        let mut running: HashMap<AgentId, AgentConfig> = HashMap::new();

        loop {
            match command_rx.recv() {
                Ok(PoolCommand::Spawn(id, config)) => {
                    let _ = event_tx.send(AgentEvent::Spawned(id));
                    let _ = event_tx.send(AgentEvent::Started(id));
                    running.insert(id, config);
                }
                Ok(PoolCommand::Stop(id)) => {
                    if running.remove(&id).is_some() {
                        let _ = event_tx.send(AgentEvent::Stopped(id));
                    }
                }
                Ok(PoolCommand::StopAll) => {
                    for id in running.keys().copied().collect::<Vec<_>>() {
                        let _ = event_tx.send(AgentEvent::Stopped(id));
                    }
                    running.clear();
                }
                Ok(PoolCommand::Shutdown) | Err(_) => break,
            }
        }
    }

    pub fn spawn(&mut self, config: AgentConfig) -> AgentId {
        let id = AgentId::new(self.next_id);
        self.next_id += 1;

        let task = AgentTask {
            id,
            config: config.clone(),
            state: AgentState::Pending,
        };
        self.tasks.insert(id, task);

        let _ = self.command_tx.send(PoolCommand::Spawn(id, config));
        id
    }

    pub fn spawn_many(&mut self, configs: Vec<AgentConfig>) -> Vec<AgentId> {
        configs.into_iter().map(|c| self.spawn(c)).collect()
    }

    pub fn stop(&mut self, id: AgentId) {
        if self.tasks.remove(&id).is_some() {
            let _ = self.command_tx.send(PoolCommand::Stop(id));
        }
    }

    pub fn stop_all(&mut self) {
        self.tasks.clear();
        let _ = self.command_tx.send(PoolCommand::StopAll);
    }

    /// Stop the most recently spawned active agent
    pub fn stop_current(&mut self) -> Option<AgentId> {
        let id = self.tasks
            .values()
            .filter(|t| t.state.is_active())
            .max_by_key(|t| t.id.0)
            .map(|t| t.id)?;
        self.stop(id);
        Some(id)
    }

    pub fn poll_events(&mut self) -> Vec<AgentEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            match &event {
                AgentEvent::Started(id) => {
                    if let Some(task) = self.tasks.get_mut(id) {
                        task.state = AgentState::Running;
                    }
                }
                AgentEvent::Completed(id, _) => {
                    if let Some(task) = self.tasks.get_mut(id) {
                        task.state = AgentState::Completed;
                    }
                }
                AgentEvent::Failed(id, _) => {
                    if let Some(task) = self.tasks.get_mut(id) {
                        task.state = AgentState::Failed;
                    }
                }
                AgentEvent::Stopped(id) => {
                    if let Some(task) = self.tasks.get_mut(id) {
                        task.state = AgentState::Stopped;
                    }
                }
                _ => {}
            }
            events.push(event);
        }
        events
    }

    pub fn active_count(&self) -> usize {
        self.tasks.values().filter(|t| t.state.is_active()).count()
    }

    pub fn task(&self, id: AgentId) -> Option<&AgentTask> {
        self.tasks.get(&id)
    }

    pub fn tasks(&self) -> impl Iterator<Item = &AgentTask> {
        self.tasks.values()
    }
}

impl Default for AgentPool {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AgentPool {
    fn drop(&mut self) {
        let _ = self.command_tx.send(PoolCommand::Shutdown);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}
