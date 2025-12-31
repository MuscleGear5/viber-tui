use ratatui::style::Style;

use super::palette;

pub fn border() -> Style {
    Style::default().fg(palette::BORDER)
}

pub fn border_focus() -> Style {
    Style::default().fg(palette::BORDER_FOCUS)
}

pub fn border_subtle() -> Style {
    Style::default().fg(palette::BORDER_SUBTLE)
}

pub fn title() -> Style {
    Style::default().fg(palette::CYAN)
}

pub fn text() -> Style {
    Style::default().fg(palette::TEXT_PRIMARY)
}

pub fn text_muted() -> Style {
    Style::default().fg(palette::TEXT_MUTED)
}

pub fn text_dim() -> Style {
    Style::default().fg(palette::TEXT_DIM)
}

pub fn input() -> Style {
    Style::default()
        .fg(palette::TEXT_PRIMARY)
        .bg(palette::BG_SURFACE)
}

pub fn input_cursor() -> Style {
    Style::default().fg(palette::BG_VOID).bg(palette::CYAN)
}

pub fn selected() -> Style {
    Style::default()
        .fg(palette::TEXT_PRIMARY)
        .bg(palette::BG_HIGHLIGHT)
}

pub fn highlight() -> Style {
    Style::default().fg(palette::CYAN)
}

pub fn category_mcp() -> Style {
    Style::default().fg(palette::CAT_MCP)
}

pub fn category_agent() -> Style {
    Style::default().fg(palette::CAT_AGENT)
}

pub fn category_skill() -> Style {
    Style::default().fg(palette::CAT_SKILL)
}

pub fn category_command() -> Style {
    Style::default().fg(palette::CAT_COMMAND)
}
