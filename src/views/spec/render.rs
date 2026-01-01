use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
};

use crate::theme::colors::palette;
use super::models::SpecSection;
use super::state::SpecState;

pub fn render_title(state: &SpecState, area: Rect, buf: &mut Buffer) {
    let (approved, total) = state.approval_progress();
    let progress = if total > 0 {
        format!(" [{}/{}]", approved, total)
    } else {
        String::new()
    };

    let title = format!("\u{F0219} {}{}", state.title, progress);
    let style = Style::default()
        .fg(palette::CYAN)
        .add_modifier(Modifier::BOLD);

    buf.set_string(area.x + 1, area.y, &title, style);
}

pub fn render_section_header(
    section: &SpecSection,
    selected: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let collapse_icon = if section.collapsed { "\u{F0142}" } else { "\u{F0140}" };
    let type_icon = section.section_type.icon();
    let status_icon = section.status.icon();

    let (fg, bg) = if selected {
        (palette::BG_VOID, palette::CYAN)
    } else {
        (palette::TEXT_PRIMARY, palette::BG_PANEL)
    };

    let style = Style::default().fg(fg).bg(bg);
    for x in area.x..area.x + area.width {
        buf.set_string(x, area.y, " ", style);
    }

    buf.set_string(area.x + 1, area.y, collapse_icon, style);
    buf.set_string(area.x + 3, area.y, type_icon, style);

    let title_style = style.add_modifier(Modifier::BOLD);
    buf.set_string(area.x + 5, area.y, &section.title, title_style);

    let status_style = Style::default().fg(section.status.color()).bg(bg);
    let status_x = area.x + area.width.saturating_sub(4);
    buf.set_string(status_x, area.y, status_icon, status_style);
}

pub fn render_section_content(
    section: &SpecSection,
    area: Rect,
    buf: &mut Buffer,
    scroll: usize,
) {
    if section.collapsed || area.height == 0 {
        return;
    }

    let lines: Vec<&str> = section.content.lines().collect();
    let style = Style::default().fg(palette::TEXT_SECONDARY);

    for (i, line) in lines.iter().skip(scroll).take(area.height as usize).enumerate() {
        let y = area.y + i as u16;
        let truncated = if line.len() > area.width as usize - 2 {
            &line[..area.width as usize - 2]
        } else {
            line
        };
        buf.set_string(area.x + 2, y, truncated, style);
    }
}

pub fn render_action_bar(area: Rect, buf: &mut Buffer) {
    let actions = vec![
        ("a", "Approve"),
        ("r", "Reject"),
        ("e", "Request Edit"),
        ("c", "Comments"),
        ("Enter", "Toggle"),
    ];

    let mut x = area.x + 1;
    for (key, label) in actions {
        let key_style = Style::default()
            .fg(palette::MAGENTA)
            .add_modifier(Modifier::BOLD);
        let label_style = Style::default().fg(palette::TEXT_MUTED);

        buf.set_string(x, area.y, key, key_style);
        x += key.len() as u16 + 1;
        buf.set_string(x, area.y, label, label_style);
        x += label.len() as u16 + 2;

        if x >= area.x + area.width - 10 {
            break;
        }
    }
}
