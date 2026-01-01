use crate::theme::colors::palette;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApprovalStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
    NeedsRevision,
}

impl ApprovalStatus {
    pub fn color(&self) -> Color {
        match self {
            Self::Pending => palette::TEXT_MUTED,
            Self::Approved => palette::SUCCESS,
            Self::Rejected => palette::ERROR,
            Self::NeedsRevision => palette::WARNING,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Pending => "\u{F0453}",
            Self::Approved => "\u{F05E0}",
            Self::Rejected => "\u{F0159}",
            Self::NeedsRevision => "\u{F0028}",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Pending => "Pending Review",
            Self::Approved => "Approved",
            Self::Rejected => "Rejected",
            Self::NeedsRevision => "Needs Revision",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Overview,
    Requirements,
    Architecture,
    Implementation,
    Testing,
    Custom,
}

impl SectionType {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Overview => "\u{F0219}",
            Self::Requirements => "\u{F0306}",
            Self::Architecture => "\u{F07B5}",
            Self::Implementation => "\u{F0121}",
            Self::Testing => "\u{F0493}",
            Self::Custom => "\u{F0219}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpecSection {
    pub id: String,
    pub title: String,
    pub section_type: SectionType,
    pub content: String,
    pub status: ApprovalStatus,
    pub comments: Vec<Comment>,
    pub collapsed: bool,
}

impl SpecSection {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        section_type: SectionType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            section_type,
            content: String::new(),
            status: ApprovalStatus::Pending,
            comments: Vec::new(),
            collapsed: false,
        }
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn approve(&mut self) {
        self.status = ApprovalStatus::Approved;
    }

    pub fn reject(&mut self) {
        self.status = ApprovalStatus::Rejected;
    }

    pub fn request_revision(&mut self) {
        self.status = ApprovalStatus::NeedsRevision;
    }
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub author: String,
    pub content: String,
    pub timestamp: String,
}

impl Comment {
    pub fn new(author: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            author: author.into(),
            content: content.into(),
            timestamp: String::new(),
        }
    }
}
