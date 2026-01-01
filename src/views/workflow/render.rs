use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::theme::palette::{
    BG_SURFACE, CYAN, GREEN, MAGENTA, PINK, TEXT_MUTED,
};
use super::models::{Phase, PhaseStatus};

pub fn render_phase_line(phase: &Phase, selected: bool, width: u16) -> Line<'static> {
    let status_color = match phase.status {
        PhaseStatus::Pending => TEXT_MUTED,
        PhaseStatus::InProgress => CYAN,
        PhaseStatus::Completed => GREEN,
        PhaseStatus::Failed => PINK,
        PhaseStatus::Skipped => TEXT_MUTED,
    };

    let icon = phase.status.icon();
    let progress_str = format!("{:>3}%", phase.progress);
    let name_width = width.saturating_sub(12) as usize;
    let name = if phase.name.len() > name_width {
        format!("{}…", &phase.name[..name_width.saturating_sub(1)])
    } else {
        format!("{:<width$}", phase.name, width = name_width)
    };

    let base_style = if selected {
        Style::default().fg(status_color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(status_color)
    };

    let selector = if selected { "\u{F0142} " } else { "  " };

    Line::from(vec![
        Span::styled(selector, base_style),
        Span::styled(format!("{} ", icon), base_style),
        Span::styled(name, base_style),
        Span::styled(progress_str, Style::default().fg(MAGENTA)),
    ])
}

pub fn render_dag_connection(buf: &mut Buffer, area: Rect, from_y: u16, to_y: u16) {
    if area.width < 3 || from_y >= area.height || to_y >= area.height {
        return;
    }

    let x = area.x + 1;
    let connector_style = Style::default().fg(CYAN);

    if to_y > from_y {
        for y in (area.y + from_y + 1)..=(area.y + to_y) {
            if y < area.y + area.height {
                buf[(x, y)].set_char('│').set_style(connector_style);
            }
        }
    }
}

pub fn render_progress_bar(buf: &mut Buffer, area: Rect, progress: u8) {
    if area.width < 4 {
        return;
    }

    let filled = ((progress as u16 * (area.width - 2)) / 100) as u16;
    let empty = area.width - 2 - filled;

    let bar_style = Style::default().fg(GREEN);
    let empty_style = Style::default().fg(BG_SURFACE);

    buf[(area.x, area.y)].set_char('[').set_style(bar_style);

    for i in 0..filled {
        buf[(area.x + 1 + i, area.y)].set_char('█').set_style(bar_style);
    }

    for i in 0..empty {
        buf[(area.x + 1 + filled + i, area.y)].set_char('░').set_style(empty_style);
    }

    buf[(area.x + area.width - 1, area.y)].set_char(']').set_style(bar_style);
}
