use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::theme::colors::palette;
use super::models::{Question, ValidationResult};

pub fn render_question_prompt(question: &Question, area: Rect, buf: &mut Buffer, selected: bool) {
    let style = if selected {
        Style::default().fg(palette::CYAN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette::TEXT_PRIMARY)
    };
    let required_marker = if question.required { " *" } else { "" };
    let text = format!("{}{}", question.prompt, required_marker);
    buf.set_string(area.x, area.y, &text, style);
}

pub fn render_text_input(value: &str, area: Rect, buf: &mut Buffer, focused: bool) {
    let style = if focused {
        Style::default().fg(palette::TEXT_PRIMARY).bg(palette::BG_ACTIVE)
    } else {
        Style::default().fg(palette::TEXT_SECONDARY).bg(palette::BG_SURFACE)
    };
    let display = if value.is_empty() && !focused {
        "Enter text...".to_string()
    } else {
        format!("{}_", value)
    };
    let width = area.width as usize;
    let truncated: String = display.chars().take(width.saturating_sub(1)).collect();
    buf.set_string(area.x, area.y, &truncated, style);
}

pub fn render_choice_list(choices: &[(String, String, bool)], area: Rect, buf: &mut Buffer, focused_idx: Option<usize>) {
    for (i, (_, label, selected)) in choices.iter().enumerate() {
        if i as u16 >= area.height {
            break;
        }
        let y = area.y + i as u16;
        let marker = if *selected { "\u{F0C52}" } else { "\u{F0130}" };
        let is_focused = focused_idx == Some(i);
        let style = if is_focused {
            Style::default().fg(palette::CYAN).add_modifier(Modifier::BOLD)
        } else if *selected {
            Style::default().fg(palette::GREEN)
        } else {
            Style::default().fg(palette::TEXT_SECONDARY)
        };
        buf.set_string(area.x, y, marker, style);
        buf.set_string(area.x + 2, y, label, style);
    }
}

pub fn render_validation_message(result: &ValidationResult, area: Rect, buf: &mut Buffer) {
    let (text, color) = match result {
        ValidationResult::Valid => ("\u{F00C0} Valid", palette::SUCCESS),
        ValidationResult::Invalid(msg) => (msg.as_str(), palette::ERROR),
        ValidationResult::Pending => ("", palette::TEXT_MUTED),
    };
    if !text.is_empty() {
        buf.set_string(area.x, area.y, text, Style::default().fg(color));
    }
}

pub fn render_progress_bar(current: usize, total: usize, area: Rect, buf: &mut Buffer) {
    if total == 0 || area.width < 10 {
        return;
    }
    let progress = current as f64 / total as f64;
    let filled_width = ((area.width as f64 - 2.0) * progress) as u16;
    let label = format!("{}/{}", current + 1, total);
    buf.set_string(area.x, area.y, "[", Style::default().fg(palette::BORDER));
    for i in 0..area.width.saturating_sub(2) {
        let ch = if i < filled_width { "\u{2588}" } else { "\u{2591}" };
        let color = if i < filled_width { palette::CYAN } else { palette::BG_SURFACE };
        buf.set_string(area.x + 1 + i, area.y, ch, Style::default().fg(color));
    }
    buf.set_string(area.x + area.width.saturating_sub(1), area.y, "]", Style::default().fg(palette::BORDER));
    let label_x = area.x + (area.width.saturating_sub(label.len() as u16)) / 2;
    buf.set_string(label_x, area.y, &label, Style::default().fg(palette::TEXT_PRIMARY).add_modifier(Modifier::BOLD));
}

pub fn render_confirm_buttons(confirmed: bool, area: Rect, buf: &mut Buffer, focused: bool) {
    let yes_style = if confirmed {
        Style::default().fg(palette::BG_PANEL).bg(palette::GREEN).add_modifier(Modifier::BOLD)
    } else if focused {
        Style::default().fg(palette::GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette::TEXT_SECONDARY)
    };
    let no_style = if !confirmed {
        Style::default().fg(palette::BG_PANEL).bg(palette::PINK).add_modifier(Modifier::BOLD)
    } else if focused {
        Style::default().fg(palette::PINK).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette::TEXT_SECONDARY)
    };
    buf.set_string(area.x, area.y, " Yes ", yes_style);
    buf.set_string(area.x + 7, area.y, " No ", no_style);
}
