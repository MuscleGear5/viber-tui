use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl ToastLevel {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "\u{F064E}",
            Self::Success => "\u{F00C0}",
            Self::Warning => "\u{F0028}",
            Self::Error => "\u{F0159}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastAction {
    pub label: String,
    pub id: String,
}

impl ToastAction {
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self { label: label.into(), id: id.into() }
    }
}

#[derive(Debug, Clone)]
pub struct ToastNotification {
    pub id: u64,
    pub message: String,
    pub level: ToastLevel,
    pub created: Instant,
    pub duration_ms: u64,
    pub actions: Vec<ToastAction>,
    pub dismissible: bool,
}

impl ToastNotification {
    pub fn new(id: u64, message: impl Into<String>, level: ToastLevel) -> Self {
        Self {
            id,
            message: message.into(),
            level,
            created: Instant::now(),
            duration_ms: 3000,
            actions: Vec::new(),
            dismissible: true,
        }
    }

    pub fn with_duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    pub fn with_action(mut self, action: ToastAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn persistent(mut self) -> Self {
        self.duration_ms = u64::MAX;
        self
    }

    pub fn is_expired(&self) -> bool {
        self.created.elapsed().as_millis() as u64 >= self.duration_ms
    }

    pub fn progress(&self) -> f32 {
        if self.duration_ms == u64::MAX { return 0.0; }
        (self.created.elapsed().as_millis() as f32 / self.duration_ms as f32).min(1.0)
    }

    pub fn slide_progress(&self) -> f32 {
        (self.created.elapsed().as_millis() as f32 / 200.0).min(1.0)
    }
}
