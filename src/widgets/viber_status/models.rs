//! VIBER status models - phase, powers, and vibe level.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViberPhase {
    #[default]
    Idle,
    IdeaCapture,
    Decomposition,
    Questionnaire,
    SpecGeneration,
    TaskDecomposition,
    Scaffold,
    Implementation,
    Polish,
    Validation,
    Delivery,
}

impl ViberPhase {
    pub fn number(&self) -> Option<u8> {
        match self {
            Self::Idle => None,
            Self::IdeaCapture => Some(0),
            Self::Decomposition => Some(1),
            Self::Questionnaire => Some(2),
            Self::SpecGeneration => Some(3),
            Self::TaskDecomposition => Some(4),
            Self::Scaffold => Some(5),
            Self::Implementation => Some(6),
            Self::Polish => Some(7),
            Self::Validation => Some(8),
            Self::Delivery => Some(9),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "IDLE",
            Self::IdeaCapture => "IDEA",
            Self::Decomposition => "DECOMP",
            Self::Questionnaire => "QUEST",
            Self::SpecGeneration => "SPEC",
            Self::TaskDecomposition => "TASKS",
            Self::Scaffold => "SCAFFOLD",
            Self::Implementation => "IMPL",
            Self::Polish => "POLISH",
            Self::Validation => "VALIDATE",
            Self::Delivery => "DELIVER",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Idle => "\u{F0209}",           // nf-md-sleep
            Self::IdeaCapture => "\u{F0EB}",     // nf-fa-lightbulb_o
            Self::Decomposition => "\u{F0493}",  // nf-md-file_tree
            Self::Questionnaire => "\u{F059}",   // nf-fa-question_circle
            Self::SpecGeneration => "\u{F0219}", // nf-md-file_document
            Self::TaskDecomposition => "\u{F0AE}",// nf-fa-tasks
            Self::Scaffold => "\u{F1898}",       // nf-md-folder_plus
            Self::Implementation => "\u{F121}",  // nf-fa-code
            Self::Polish => "\u{F0D10}",         // nf-md-broom
            Self::Validation => "\u{F00C0}",     // nf-md-check
            Self::Delivery => "\u{F0753}",       // nf-md-rocket_launch
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViberPower {
    Observe,
    Stop,
    Undo,
    Inject,
    Redirect,
    Protect,
}

impl ViberPower {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Observe => "\u{F06D0}",  // nf-md-eye
            Self::Stop => "\u{F04DB}",     // nf-md-stop
            Self::Undo => "\u{F054C}",     // nf-md-undo
            Self::Inject => "\u{F0214}",   // nf-md-needle
            Self::Redirect => "\u{F064D}", // nf-md-ray_start_arrow
            Self::Protect => "\u{F0512}",  // nf-md-shield
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Observe => "OBSERVE",
            Self::Stop => "STOP",
            Self::Undo => "UNDO",
            Self::Inject => "INJECT",
            Self::Redirect => "REDIRECT",
            Self::Protect => "PROTECT",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VibeLevel {
    #[default]
    Nominal,
    Drifting,
    Warning,
    Critical,
}

impl VibeLevel {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Nominal => "\u{F00C0}",   // nf-md-check
            Self::Drifting => "\u{F0028}",  // nf-md-alert
            Self::Warning => "\u{F0026}",   // nf-md-alert_circle
            Self::Critical => "\u{F0159}",  // nf-md-close
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Nominal => "NOMINAL",
            Self::Drifting => "DRIFTING",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
        }
    }
}
