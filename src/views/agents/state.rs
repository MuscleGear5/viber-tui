use crate::theme::colors::palette;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Working,
    Waiting,
    Error,
}

impl AgentStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Idle => "\u{F0142}",    // nf-md-circle_outline
            Self::Working => "\u{F0F47}", // nf-md-lightning_bolt
            Self::Waiting => "\u{F0954}", // nf-md-timer_sand
            Self::Error => "\u{F0159}",   // nf-md-close_circle
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Idle => palette::TEXT_MUTED,
            Self::Working => palette::CYAN,
            Self::Waiting => palette::MAGENTA,
            Self::Error => palette::PINK,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub status: AgentStatus,
    pub memory_mb: u32,
    pub memory_max_mb: u32,
    pub tokens_used: u32,
    pub tokens_max: u32,
    pub current_task: Option<String>,
}

impl Agent {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            status: AgentStatus::Idle,
            memory_mb: 0,
            memory_max_mb: 512,
            tokens_used: 0,
            tokens_max: 100_000,
            current_task: None,
        }
    }

    pub fn memory_percent(&self) -> f64 {
        if self.memory_max_mb == 0 {
            return 0.0;
        }
        (self.memory_mb as f64 / self.memory_max_mb as f64) * 100.0
    }

    pub fn tokens_percent(&self) -> f64 {
        if self.tokens_max == 0 {
            return 0.0;
        }
        (self.tokens_used as f64 / self.tokens_max as f64) * 100.0
    }
}

#[derive(Debug, Default)]
pub struct AgentsState {
    pub agents: Vec<Agent>,
    pub selected: usize,
    pub show_detail: bool,
}

impl AgentsState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_next(&mut self) {
        if !self.agents.is_empty() {
            self.selected = (self.selected + 1) % self.agents.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.agents.is_empty() {
            self.selected = self.selected.checked_sub(1).unwrap_or(self.agents.len() - 1);
        }
    }

    pub fn selected_agent(&self) -> Option<&Agent> {
        self.agents.get(self.selected)
    }

    pub fn toggle_detail(&mut self) {
        self.show_detail = !self.show_detail;
    }
}
