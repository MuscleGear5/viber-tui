use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(pub u64);

impl AgentId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentKind {
    Coder,
    Reviewer,
    Tester,
    Debugger,
    Documenter,
    Planner,
    Explorer,
    Custom,
}

impl AgentKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Coder => "\u{F121}",
            Self::Reviewer => "\u{F06E}",
            Self::Tester => "\u{F0668}",
            Self::Debugger => "\u{F188}",
            Self::Documenter => "\u{F0219}",
            Self::Planner => "\u{F073}",
            Self::Explorer => "\u{F002}",
            Self::Custom => "\u{F085}",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Coder => "Coder",
            Self::Reviewer => "Reviewer",
            Self::Tester => "Tester",
            Self::Debugger => "Debugger",
            Self::Documenter => "Documenter",
            Self::Planner => "Planner",
            Self::Explorer => "Explorer",
            Self::Custom => "Custom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Stopped,
}

impl AgentState {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Pending => "\u{F0028}",
            Self::Running => "\u{F04B}",
            Self::Paused => "\u{F04C}",
            Self::Completed => "\u{F00C0}",
            Self::Failed => "\u{F0159}",
            Self::Stopped => "\u{F04D}",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Running | Self::Paused)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Stopped)
    }
}

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub kind: AgentKind,
    pub name: String,
    pub task: String,
    pub timeout: Option<Duration>,
    pub max_retries: u32,
}

impl AgentConfig {
    pub fn new(kind: AgentKind, name: impl Into<String>, task: impl Into<String>) -> Self {
        Self { kind, name: name.into(), task: task.into(), timeout: Some(Duration::from_secs(300)), max_retries: 3 }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self { self.timeout = Some(timeout); self }
}

#[derive(Debug, Clone, Default)]
pub struct AgentMetrics {
    pub tokens_used: u64,
    pub api_calls: u32,
    pub files_modified: u32,
    pub errors: u32,
}

#[derive(Debug, Clone)]
pub struct AgentInstance {
    pub id: AgentId,
    pub config: AgentConfig,
    pub state: AgentState,
    pub started_at: Option<Instant>,
    pub metrics: AgentMetrics,
    pub output_buffer: Vec<String>,
    pub retry_count: u32,
}

impl AgentInstance {
    pub fn new(id: AgentId, config: AgentConfig) -> Self {
        Self { id, config, state: AgentState::Pending, started_at: None, metrics: AgentMetrics::default(), output_buffer: Vec::new(), retry_count: 0 }
    }

    pub fn elapsed(&self) -> Option<Duration> { self.started_at.map(|s| s.elapsed()) }

    pub fn is_timed_out(&self) -> bool {
        matches!((self.config.timeout, self.elapsed()), (Some(t), Some(e)) if e > t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_state_behavior() {
        assert!(!AgentState::Pending.is_active());
        assert!(AgentState::Running.is_active());
        assert!(AgentState::Completed.is_terminal());
        assert!(!AgentState::Running.is_terminal());
    }

    #[test]
    fn agent_instance_lifecycle() {
        let config = AgentConfig::new(AgentKind::Coder, "coder", "task");
        let agent = AgentInstance::new(AgentId::new(1), config);
        assert_eq!(agent.state, AgentState::Pending);
        assert!(!agent.is_timed_out());
    }
}
