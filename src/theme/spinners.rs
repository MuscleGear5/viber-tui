pub const SPINNER_DOTS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const SPINNER_LINE: &[&str] = &["-", "\\", "|", "/"];
pub const SPINNER_CIRCLE: &[&str] = &["◐", "◓", "◑", "◒"];
pub const SPINNER_SQUARE: &[&str] = &["◰", "◳", "◲", "◱"];
pub const SPINNER_TRIANGLE: &[&str] = &["◢", "◣", "◤", "◥"];
pub const SPINNER_BOUNCE: &[&str] = &["⠁", "⠂", "⠄", "⠂"];
pub const SPINNER_GROW: &[&str] = &["▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃"];
pub const SPINNER_PULSE: &[&str] = &["█", "▓", "▒", "░", "▒", "▓"];
pub const SPINNER_ORBIT: &[&str] = &["◜ ", " ◝", " ◞", "◟ "];
pub const SPINNER_STAR: &[&str] = &["✶", "✸", "✹", "✺", "✹", "✸"];
pub const SPINNER_NEON: &[&str] = &["\u{F1110}", "\u{F1111}", "\u{F1112}", "\u{F1113}"];
pub const SPINNER_CLOCK: &[&str] = &["🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛"];
pub const SPINNER_BRAILLE: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerStyle {
    #[default]
    Dots,
    Line,
    Circle,
    Square,
    Triangle,
    Bounce,
    Grow,
    Pulse,
    Orbit,
    Star,
    Neon,
    Clock,
    Braille,
}

impl SpinnerStyle {
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            Self::Dots => SPINNER_DOTS,
            Self::Line => SPINNER_LINE,
            Self::Circle => SPINNER_CIRCLE,
            Self::Square => SPINNER_SQUARE,
            Self::Triangle => SPINNER_TRIANGLE,
            Self::Bounce => SPINNER_BOUNCE,
            Self::Grow => SPINNER_GROW,
            Self::Pulse => SPINNER_PULSE,
            Self::Orbit => SPINNER_ORBIT,
            Self::Star => SPINNER_STAR,
            Self::Neon => SPINNER_NEON,
            Self::Clock => SPINNER_CLOCK,
            Self::Braille => SPINNER_BRAILLE,
        }
    }

    pub fn frame(&self, tick: usize) -> &'static str {
        let frames = self.frames();
        frames[tick % frames.len()]
    }

    pub fn speed_divisor(&self) -> u64 {
        match self {
            Self::Dots | Self::Braille => 3,
            Self::Line | Self::Circle | Self::Square | Self::Triangle => 4,
            Self::Bounce | Self::Orbit => 5,
            Self::Grow | Self::Pulse | Self::Star => 2,
            Self::Neon | Self::Clock => 6,
        }
    }
}

pub struct Spinner {
    style: SpinnerStyle,
    frame: usize,
    label: Option<String>,
}

impl Spinner {
    pub fn new(style: SpinnerStyle) -> Self {
        Self { style, frame: 0, label: None }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn tick(&mut self) {
        let frames = self.style.frames();
        self.frame = (self.frame + 1) % frames.len();
    }

    pub fn current_frame(&self) -> &'static str {
        self.style.frame(self.frame)
    }

    pub fn render(&self) -> String {
        match &self.label {
            Some(l) => format!("{} {}", self.current_frame(), l),
            None => self.current_frame().to_string(),
        }
    }
}

pub struct ProgressBar {
    width: usize,
    progress: f32,
    style: ProgressStyle,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ProgressStyle {
    #[default]
    Block,
    Gradient,
    Pulse,
}

const BLOCKS: &[char] = &[' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

impl ProgressBar {
    pub fn new(width: usize) -> Self {
        Self { width, progress: 0.0, style: ProgressStyle::Block }
    }

    pub fn with_style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn render(&self) -> String {
        let filled = (self.progress * self.width as f32) as usize;
        let partial = ((self.progress * self.width as f32).fract() * 8.0) as usize;
        let mut bar = String::with_capacity(self.width);
        for i in 0..self.width {
            if i < filled {
                bar.push('█');
            } else if i == filled && partial > 0 {
                bar.push(BLOCKS[partial]);
            } else {
                bar.push('░');
            }
        }
        bar
    }
}
