use ratatui::style::{Color, Style};

use crate::data::ActionCategory;

pub mod palette {
    use ratatui::style::Color;

    pub const BG_VOID: Color = Color::Rgb(10, 10, 18);
    pub const BG_PANEL: Color = Color::Rgb(18, 18, 30);
    pub const BG_SURFACE: Color = Color::Rgb(25, 25, 42);
    pub const BG_HIGHLIGHT: Color = Color::Rgb(35, 30, 55);
    pub const BG_ACTIVE: Color = Color::Rgb(45, 35, 70);
    pub const BG_ELEVATED: Color = Color::Rgb(30, 28, 48);
    pub const BORDER: Color = Color::Rgb(60, 50, 90);
    pub const BORDER_FOCUS: Color = Color::Rgb(100, 80, 160);
    pub const BORDER_SUBTLE: Color = Color::Rgb(50, 45, 75);
    pub const BORDER_DIM: Color = Color::Rgb(40, 35, 60);

    pub const CYAN: Color = Color::Rgb(0, 255, 255);
    pub const CYAN_DIM: Color = Color::Rgb(0, 180, 200);
    pub const MAGENTA: Color = Color::Rgb(255, 0, 255);
    pub const MAGENTA_DIM: Color = Color::Rgb(200, 0, 200);
    pub const PINK: Color = Color::Rgb(255, 51, 153);
    pub const PINK_DIM: Color = Color::Rgb(200, 40, 120);
    pub const PURPLE: Color = Color::Rgb(153, 102, 255);
    pub const PURPLE_DIM: Color = Color::Rgb(120, 80, 200);
    pub const GREEN: Color = Color::Rgb(0, 255, 136);
    pub const GREEN_DIM: Color = Color::Rgb(0, 180, 100);
    pub const BLUE: Color = Color::Rgb(51, 153, 255);
    pub const BLUE_DIM: Color = Color::Rgb(40, 120, 200);

    pub const TEXT_PRIMARY: Color = Color::Rgb(230, 230, 255);
    pub const TEXT_SECONDARY: Color = Color::Rgb(160, 160, 200);
    pub const TEXT_MUTED: Color = Color::Rgb(100, 100, 140);
    pub const TEXT_DIM: Color = Color::Rgb(80, 80, 110);
    pub const TEXT_DISABLED: Color = Color::Rgb(70, 70, 100);

    pub const ERROR: Color = PINK;
    pub const WARNING: Color = Color::Rgb(255, 100, 180);
    pub const SUCCESS: Color = GREEN;
    pub const INFO: Color = CYAN;

    pub const VIBER: Color = CYAN;
    pub const VIBER_DRIFT: Color = MAGENTA;
    pub const VIBER_DANGER: Color = PINK;

    pub const CAT_MCP: Color = CYAN;
    pub const CAT_AGENT: Color = MAGENTA;
    pub const CAT_SKILL: Color = PURPLE;
    pub const CAT_COMMAND: Color = GREEN;
}

pub use palette::{
    BG_ELEVATED, BG_VOID, BORDER_DIM, BORDER_SUBTLE, TEXT_DIM,
};

pub const ACCENT_CYAN: Color = palette::CYAN;
pub const ACCENT_MAGENTA: Color = palette::MAGENTA;
pub const ACCENT_PINK: Color = palette::PINK;
pub const ACCENT_PURPLE: Color = palette::PURPLE;
pub const ACCENT_GREEN: Color = palette::GREEN;
pub const ACCENT_BLUE: Color = palette::BLUE;

pub fn text_primary() -> Style {
    Style::default().fg(palette::TEXT_PRIMARY)
}

pub fn text_secondary() -> Style {
    Style::default().fg(palette::TEXT_SECONDARY)
}

pub fn text_muted() -> Style {
    Style::default().fg(palette::TEXT_MUTED)
}

pub fn text_dim() -> Style {
    Style::default().fg(palette::TEXT_DIM)
}

pub fn list_selected() -> Style {
    Style::default()
        .bg(palette::BG_HIGHLIGHT)
        .fg(palette::TEXT_PRIMARY)
}

pub fn category_color(category: ActionCategory) -> Color {
    match category {
        ActionCategory::Mcp => palette::CAT_MCP,
        ActionCategory::Agent => palette::CAT_AGENT,
        ActionCategory::Skill => palette::CAT_SKILL,
        ActionCategory::Command => palette::CAT_COMMAND,
    }
}
