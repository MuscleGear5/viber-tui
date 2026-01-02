use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalKind {
    Info,
    Warning,
    Error,
    Confirm,
    Input,
}

impl ModalKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "\u{F064E}",
            Self::Warning => "\u{F0028}",
            Self::Error => "\u{F0159}",
            Self::Confirm => "\u{F0046}",
            Self::Input => "\u{F0765}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModalButton {
    pub label: String,
    pub key: char,
    pub is_primary: bool,
    pub is_destructive: bool,
}

impl ModalButton {
    pub fn new(label: impl Into<String>, key: char) -> Self {
        Self {
            label: label.into(),
            key,
            is_primary: false,
            is_destructive: false,
        }
    }

    pub fn primary(mut self) -> Self {
        self.is_primary = true;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.is_destructive = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Modal {
    pub id: u64,
    pub kind: ModalKind,
    pub title: String,
    pub message: String,
    pub buttons: Vec<ModalButton>,
    pub input_value: Option<String>,
    pub input_placeholder: Option<String>,
    pub created_at: Instant,
    pub min_width: u16,
    pub max_width: u16,
}

impl Modal {
    pub fn new(kind: ModalKind, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: 0,
            kind,
            title: title.into(),
            message: message.into(),
            buttons: Vec::new(),
            input_value: None,
            input_placeholder: None,
            created_at: Instant::now(),
            min_width: 30,
            max_width: 60,
        }
    }

    pub fn with_button(mut self, button: ModalButton) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn with_input(mut self, placeholder: impl Into<String>) -> Self {
        self.input_value = Some(String::new());
        self.input_placeholder = Some(placeholder.into());
        self
    }

    pub fn confirm(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(ModalKind::Confirm, title, message)
            .with_button(ModalButton::new("Cancel", 'n'))
            .with_button(ModalButton::new("Confirm", 'y').primary())
    }

    pub fn destructive(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(ModalKind::Warning, title, message)
            .with_button(ModalButton::new("Cancel", 'n'))
            .with_button(ModalButton::new("Delete", 'y').destructive())
    }

    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(ModalKind::Info, title, message)
            .with_button(ModalButton::new("OK", 'o').primary())
    }

    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(ModalKind::Error, title, message)
            .with_button(ModalButton::new("OK", 'o').primary())
    }

    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
}
