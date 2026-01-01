use ratatui::style::Color;
use crate::theme::palette::{CYAN, GREEN, MAGENTA, MAGENTA_DIM, CYAN_DIM};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
    Backlog,
}

impl TaskPriority {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Critical => "\u{F0026}", // nf-md-alert_circle
            Self::High => "\u{F005D}",     // nf-md-arrow_up_bold
            Self::Medium => "\u{F0060}",   // nf-md-arrow_right_bold  
            Self::Low => "\u{F0046}",      // nf-md-arrow_down_bold
            Self::Backlog => "\u{F0142}",  // nf-md-circle_outline
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Critical => MAGENTA,
            Self::High => MAGENTA_DIM,
            Self::Medium => CYAN,
            Self::Low => CYAN_DIM,
            Self::Backlog => GREEN,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Done,
}

impl TaskStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Pending => "\u{F0142}",    // nf-md-circle_outline
            Self::InProgress => "\u{F0997}", // nf-md-progress_clock
            Self::Blocked => "\u{F0156}",    // nf-md-close_circle
            Self::Done => "\u{F0134}",       // nf-md-check_circle
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Pending => CYAN_DIM,
            Self::InProgress => CYAN,
            Self::Blocked => MAGENTA,
            Self::Done => GREEN,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub parent_id: Option<String>,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            priority: TaskPriority::Medium,
            status: TaskStatus::Pending,
            parent_id: None,
            tags: Vec::new(),
        }
    }

    pub fn priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }
}
