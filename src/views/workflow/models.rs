use std::time::Instant;

/// VIBER workflow phases - the 9-phase vibe coding pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViberPhase {
    /// Phase 0: Initial idea capture and validation
    Idea,
    /// Phase 1: Break idea into discrete components
    Decompose,
    /// Phase 2: Gather requirements via questionnaire
    Questionnaire,
    /// Phase 3: Generate OpenSpec from requirements
    SpecGen,
    /// Phase 4: Break spec into executable tasks
    TaskBreakdown,
    /// Phase 5: Scaffold project structure
    Scaffold,
    /// Phase 6: Implementation (edit cycle lives here)
    Implementation,
    /// Phase 7: Polish and refinement
    Polish,
    /// Phase 8: Final validation and delivery
    Validate,
}

impl ViberPhase {
    pub const ALL: [Self; 9] = [
        Self::Idea,
        Self::Decompose,
        Self::Questionnaire,
        Self::SpecGen,
        Self::TaskBreakdown,
        Self::Scaffold,
        Self::Implementation,
        Self::Polish,
        Self::Validate,
    ];

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Idea => "Idea",
            Self::Decompose => "Decompose",
            Self::Questionnaire => "Questionnaire",
            Self::SpecGen => "Spec Gen",
            Self::TaskBreakdown => "Task Breakdown",
            Self::Scaffold => "Scaffold",
            Self::Implementation => "Implementation",
            Self::Polish => "Polish",
            Self::Validate => "Validate",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Idea => "\u{F0EB}",          // nf-fa-lightbulb_o
            Self::Decompose => "\u{F0E8}",     // nf-fa-sitemap
            Self::Questionnaire => "\u{F128}", // nf-fa-question
            Self::SpecGen => "\u{F15C}",       // nf-fa-file_text_o
            Self::TaskBreakdown => "\u{F0AE}", // nf-fa-tasks
            Self::Scaffold => "\u{F1B3}",      // nf-fa-cubes
            Self::Implementation => "\u{F121}",// nf-fa-code
            Self::Polish => "\u{F1FC}",        // nf-fa-paint_brush
            Self::Validate => "\u{F00C}",      // nf-fa-check
        }
    }

    pub fn from_index(idx: usize) -> Option<Self> {
        Self::ALL.get(idx).copied()
    }
}

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
