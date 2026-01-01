use std::time::Instant;
use super::models::{AgentConfig, AgentId, AgentInstance, AgentState};

#[derive(Debug, Clone)]
pub enum AgentCommand {
    Spawn(AgentConfig),
    Stop(AgentId),
    Pause(AgentId),
    Resume(AgentId),
    Inject(AgentId, String),
}

#[derive(Debug, Clone)]
pub enum AgentEvent {
    Spawned(AgentId),
    Started(AgentId),
    Output(AgentId, String),
    Progress(AgentId, u8),
    Paused(AgentId),
    Resumed(AgentId),
    Completed(AgentId, String),
    Failed(AgentId, String),
    Stopped(AgentId),
    Timeout(AgentId),
}

pub trait AgentProtocol {
    fn spawn(&mut self, config: AgentConfig) -> AgentId;
    fn stop(&mut self, id: AgentId) -> bool;
    fn pause(&mut self, id: AgentId) -> bool;
    fn resume(&mut self, id: AgentId) -> bool;
    fn inject(&mut self, id: AgentId, message: String) -> bool;
    fn query(&self, id: AgentId) -> Option<&AgentInstance>;
    fn list_active(&self) -> Vec<AgentId>;
}

#[derive(Default)]
pub struct AgentController {
    next_id: u64,
    agents: Vec<AgentInstance>,
    events: Vec<AgentEvent>,
}

impl AgentController {
    pub fn new() -> Self {
        Self { next_id: 1, agents: Vec::new(), events: Vec::new() }
    }

    fn find_mut(&mut self, id: AgentId) -> Option<&mut AgentInstance> {
        self.agents.iter_mut().find(|a| a.id == id)
    }

    fn emit(&mut self, event: AgentEvent) {
        self.events.push(event);
    }

    pub fn drain_events(&mut self) -> Vec<AgentEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn check_timeouts(&mut self) -> Vec<AgentId> {
        let timed_out: Vec<_> = self.agents.iter_mut()
            .filter(|a| a.state == AgentState::Running && a.is_timed_out())
            .map(|a| { a.state = AgentState::Failed; a.id })
            .collect();
        for id in &timed_out {
            self.events.push(AgentEvent::Timeout(*id));
        }
        timed_out
    }
}

impl AgentProtocol for AgentController {
    fn spawn(&mut self, config: AgentConfig) -> AgentId {
        let id = AgentId::new(self.next_id);
        self.next_id += 1;
        let mut instance = AgentInstance::new(id, config);
        instance.state = AgentState::Running;
        instance.started_at = Some(Instant::now());
        self.agents.push(instance);
        self.emit(AgentEvent::Spawned(id));
        self.emit(AgentEvent::Started(id));
        id
    }

    fn stop(&mut self, id: AgentId) -> bool {
        if let Some(agent) = self.find_mut(id) {
            if agent.state.is_active() {
                agent.state = AgentState::Stopped;
                self.emit(AgentEvent::Stopped(id));
                return true;
            }
        }
        false
    }

    fn pause(&mut self, id: AgentId) -> bool {
        if let Some(agent) = self.find_mut(id) {
            if agent.state == AgentState::Running {
                agent.state = AgentState::Paused;
                self.emit(AgentEvent::Paused(id));
                return true;
            }
        }
        false
    }

    fn resume(&mut self, id: AgentId) -> bool {
        if let Some(agent) = self.find_mut(id) {
            if agent.state == AgentState::Paused {
                agent.state = AgentState::Running;
                self.emit(AgentEvent::Resumed(id));
                return true;
            }
        }
        false
    }

    fn inject(&mut self, id: AgentId, message: String) -> bool {
        if let Some(agent) = self.find_mut(id) {
            if agent.state.is_active() {
                agent.output_buffer.push(format!("[INJECT] {}", message));
                return true;
            }
        }
        false
    }

    fn query(&self, id: AgentId) -> Option<&AgentInstance> {
        self.agents.iter().find(|a| a.id == id)
    }

    fn list_active(&self) -> Vec<AgentId> {
        self.agents.iter().filter(|a| a.state.is_active()).map(|a| a.id).collect()
    }
}
