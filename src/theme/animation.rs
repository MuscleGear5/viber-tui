use std::f32::consts::PI;

pub const TICK_RATE_MS: u64 = 33;

#[derive(Debug, Clone)]
pub struct AnimationState {
    tick: u64,
    viber_eye_frame: usize,
    spinner_frame: usize,
    dot_spinner_frame: usize,
    pulse_intensity: f32,
    pulse_secondary: f32,
    cursor_visible: bool,
    wave_offset: usize,
    glow_cycle: f32,
    loading_pos: usize,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            viber_eye_frame: 0,
            spinner_frame: 0,
            dot_spinner_frame: 0,
            pulse_intensity: 0.5,
            pulse_secondary: 0.0,
            cursor_visible: true,
            wave_offset: 0,
            glow_cycle: 0.0,
            loading_pos: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);

        if self.tick % 18 == 0 {
            self.viber_eye_frame = (self.viber_eye_frame + 1) % 8;
        }

        if self.tick % 3 == 0 {
            self.spinner_frame = (self.spinner_frame + 1) % 4;
        }

        if self.tick % 6 == 0 {
            self.dot_spinner_frame = (self.dot_spinner_frame + 1) % 4;
        }

        let phase = (self.tick as f32 * 0.1).sin();
        self.pulse_intensity = (phase + 1.0) / 2.0;

        let phase2 = ((self.tick as f32 * 0.1) + (PI / 2.0)).sin();
        self.pulse_secondary = (phase2 + 1.0) / 2.0;

        if self.tick % 16 == 0 {
            self.cursor_visible = !self.cursor_visible;
        }

        if self.tick % 15 == 0 {
            self.wave_offset = (self.wave_offset + 1) % 8;
        }

        self.glow_cycle = (self.tick as f32 * 0.05).sin().abs();

        if self.tick % 2 == 0 {
            self.loading_pos = (self.loading_pos + 1) % 100;
        }
    }

    pub fn viber_eye(&self) -> &'static str {
        const EYES: [&str; 8] = ["👁", "👁", "👁", "◉", "◎", "○", "◎", "◉"];
        EYES[self.viber_eye_frame]
    }

    pub fn viber_eye_text(&self) -> &'static str {
        const EYES: [&str; 8] = ["(O)", "(O)", "(o)", "(·)", "(-)", "(·)", "(o)", "(O)"];
        EYES[self.viber_eye_frame]
    }

    pub fn spinner(&self) -> &'static str {
        const SPINNERS: [&str; 4] = ["◐", "◓", "◑", "◒"];
        SPINNERS[self.spinner_frame]
    }

    pub fn spinner_braille(&self) -> &'static str {
        const SPINNERS: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
        SPINNERS[self.tick as usize % 8]
    }

    pub fn spinner_dots(&self) -> &'static str {
        const DOTS: [&str; 4] = ["·  ", "·· ", "···", " ··"];
        DOTS[self.dot_spinner_frame]
    }

    pub fn pulse(&self) -> f32 {
        self.pulse_intensity
    }

    pub fn pulse_secondary(&self) -> f32 {
        self.pulse_secondary
    }

    pub fn glow(&self) -> f32 {
        self.glow_cycle
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn cursor(&self) -> &'static str {
        if self.cursor_visible {
            "█"
        } else {
            " "
        }
    }

    pub fn cursor_thin(&self) -> &'static str {
        if self.cursor_visible {
            "▏"
        } else {
            " "
        }
    }

    pub fn vibe_wave(&self) -> &'static str {
        const WAVE: &str = "∿≋∿≋∿≋∿≋∿≋∿≋∿≋∿≋";
        let start = self.wave_offset;
        let end = start + 8;
        &WAVE[start..end.min(WAVE.len())]
    }

    pub fn vibe_wave_short(&self) -> &'static str {
        const WAVES: [&str; 4] = ["∿≋∿≋", "≋∿≋∿", "∿≋∿≋", "≋∿≋∿"];
        WAVES[self.wave_offset % 4]
    }

    pub fn progress_bar(&self, width: usize, progress: f32) -> String {
        let filled = (width as f32 * progress.clamp(0.0, 1.0)) as usize;
        let empty = width.saturating_sub(filled);
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    pub fn loading_bar(&self, width: usize) -> String {
        let mut chars: Vec<char> = vec!['░'; width];
        let pos = (self.loading_pos * width / 100) % width;
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
            AgentStatus::Waiting => "◦",
            AgentStatus::Paused => "⏸",
            AgentStatus::Done => "✓",
            AgentStatus::Error => "✗",
        }
    }

    pub fn tick_count(&self) -> u64 {
        self.tick
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Running,
    Thinking,
    Waiting,
    Paused,
    Done,
    Error,
}
