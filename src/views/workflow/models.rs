use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

impl PhaseStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Pending => "\u{F0142}",
            Self::InProgress => "\u{F0E4E}",
            Self::Completed => "\u{F0134}",
            Self::Failed => "\u{F0159}",
            Self::Skipped => "\u{F04DB}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Phase {
    pub id: usize,
    pub name: String,
    pub status: PhaseStatus,
    pub progress: u8,
    pub dependencies: Vec<usize>,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
}

impl Phase {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            status: PhaseStatus::Pending,
            progress: 0,
            dependencies: Vec::new(),
            started_at: None,
            completed_at: None,
        }
    }

    pub fn with_deps(mut self, deps: Vec<usize>) -> Self {
        self.dependencies = deps;
        self
    }

    pub fn start(&mut self) {
        self.status = PhaseStatus::InProgress;
        self.started_at = Some(Instant::now());
    }

    pub fn complete(&mut self) {
        self.status = PhaseStatus::Completed;
        self.progress = 100;
        self.completed_at = Some(Instant::now());
    }

    pub fn fail(&mut self) {
        self.status = PhaseStatus::Failed;
        self.completed_at = Some(Instant::now());
    }
}
