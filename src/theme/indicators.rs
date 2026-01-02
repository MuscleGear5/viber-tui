use super::AnimationState;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Running,
    Thinking,
    Waiting,
    Paused,
    Done,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
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
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub created: Instant,
    pub duration_ms: u64,
}

impl Toast {
    pub fn new(message: impl Into<String>, level: ToastLevel) -> Self {
        Self { message: message.into(), level, created: Instant::now(), duration_ms: 3000 }
    }
    pub fn info(msg: impl Into<String>) -> Self { Self::new(msg, ToastLevel::Info) }
    pub fn success(msg: impl Into<String>) -> Self { Self::new(msg, ToastLevel::Success) }
    pub fn warning(msg: impl Into<String>) -> Self { Self::new(msg, ToastLevel::Warning) }
    pub fn error(msg: impl Into<String>) -> Self { Self::new(msg, ToastLevel::Error) }
    pub fn is_expired(&self) -> bool { self.created.elapsed().as_millis() as u64 >= self.duration_ms }
}

#[derive(Debug, Default)]
pub struct ToastManager {
    toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, toast: Toast) { self.toasts.push(toast); }
    pub fn tick(&mut self) { self.toasts.retain(|t| !t.is_expired()); }
    pub fn active(&self) -> Option<&Toast> { self.toasts.first() }
    pub fn count(&self) -> usize { self.toasts.len() }
}

const EYES_ICON: [&str; 8] = [
    "\u{f06e}", "\u{f06e}", "\u{f06e}", "\u{25C9}", "\u{25CE}", "\u{25CB}", "\u{25CE}", "\u{25C9}",
];
const EYES_TEXT: [&str; 8] = ["(O)", "(O)", "(o)", "(·)", "(-)", "(·)", "(o)", "(O)"];
const SPINNERS: [&str; 4] = ["◐", "◓", "◑", "◒"];
const BRAILLE: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
const DOTS: [&str; 4] = ["·  ", "·· ", "···", " ··"];
const WAVES: [&str; 4] = ["∿≋∿≋", "≋∿≋∿", "∿≋∿≋", "≋∿≋∿"];

impl AnimationState {
    pub fn viber_eye(&self) -> &'static str {
        EYES_ICON[self.viber_eye_frame()]
    }

    pub fn viber_eye_text(&self) -> &'static str {
        EYES_TEXT[self.viber_eye_frame()]
    }

    pub fn spinner(&self) -> &'static str {
        SPINNERS[self.spinner_frame()]
    }

    pub fn spinner_braille(&self) -> &'static str {
        BRAILLE[self.tick_count() as usize % 8]
    }

    pub fn spinner_dots(&self) -> &'static str {
        DOTS[self.dot_spinner_frame()]
    }

    pub fn cursor(&self) -> &'static str {
        if self.cursor_visible() {
            "█"
        } else {
            " "
        }
    }

    pub fn cursor_thin(&self) -> &'static str {
        if self.cursor_visible() {
            "▏"
        } else {
            " "
        }
    }

    pub fn vibe_wave_short(&self) -> &'static str {
        WAVES[self.wave_offset() % 4]
    }

    pub fn progress_bar(&self, width: usize, progress: f32) -> String {
        let filled = (width as f32 * progress.clamp(0.0, 1.0)) as usize;
        let empty = width.saturating_sub(filled);
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    pub fn loading_bar(&self, width: usize) -> String {
        let mut chars: Vec<char> = vec!['░'; width];
        let pos = (self.loading_pos() * width / 100) % width;
        let highlight_width = 3.min(width);

        for i in 0..highlight_width {
            let idx = (pos + i) % width;
            chars[idx] = '█';
        }

        chars.into_iter().collect()
    }

    pub fn status_indicator(&self, status: AgentStatus) -> &'static str {
        match status {
            AgentStatus::Running => self.spinner(),
            AgentStatus::Thinking => self.spinner_braille(),
            AgentStatus::Waiting => "\u{25E6}",
            AgentStatus::Paused => "\u{eae8}",
            AgentStatus::Done => "\u{eab2}",
            AgentStatus::Error => "\u{ea87}",
        }
    }
}
