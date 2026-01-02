use super::models::{AgentConfig, AgentId};

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

#[derive(Debug, Clone)]
pub struct DispatchResult {
    pub agent_ids: Vec<AgentId>,
    pub failed: Vec<String>,
}

impl DispatchResult {
    pub fn success_count(&self) -> usize {
        self.agent_ids.len()
    }

    pub fn failure_count(&self) -> usize {
        self.failed.len()
    }

    pub fn is_partial_success(&self) -> bool {
        !self.agent_ids.is_empty() && !self.failed.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AgentStatusSummary {
    pub pending: usize,
    pub running: usize,
    pub paused: usize,
    pub completed: usize,
    pub failed: usize,
    pub stopped: usize,
}

impl AgentStatusSummary {
    pub fn total(&self) -> usize {
        self.pending + self.running + self.paused + self.completed + self.failed + self.stopped
    }

    pub fn active(&self) -> usize {
        self.running + self.paused
    }

    pub fn terminal(&self) -> usize {
        self.completed + self.failed + self.stopped
    }
}
