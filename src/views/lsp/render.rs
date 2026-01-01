use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::theme::palette;

use super::models::{Diagnostic, HoverInfo, Reference};
use super::state::LspState;

pub fn render_diagnostic_line(diag: &Diagnostic, selected: bool, area: Rect, buf: &mut Buffer) {
    if area.height == 0 {
        return;
    }

    let bg = if selected { palette::BG_ACTIVE } else { palette::BG_PANEL };
    let style = Style::default().bg(bg);

    for x in area.x..area.x.saturating_add(area.width) {
        buf[(x, area.y)].set_style(style);
    }

    let icon_span = Span::styled(
        diag.severity.icon(),
        Style::default().fg(diag.severity.color()),
    );

    let loc_span = Span::styled(
        format!(" {}:{} ", diag.line, diag.column),
        Style::default().fg(palette::TEXT_MUTED),
    );

    let msg_span = Span::styled(&diag.message, Style::default().fg(palette::TEXT_PRIMARY));

    let line = Line::from(vec![icon_span, loc_span, msg_span]);
    buf.set_line(area.x.saturating_add(1), area.y, &line, area.width.saturating_sub(2));
}

pub fn render_hover_content(hover: &HoverInfo, area: Rect, buf: &mut Buffer) {
    if area.height == 0 || area.width == 0 {
        return;
    }

    let bg_style = Style::default().bg(palette::BG_ELEVATED);
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            buf[(x, y)].set_style(bg_style);
        }
    }

    let lines: Vec<&str> = hover.content.lines().collect();
    let max_lines = area.height as usize;

    for (i, content) in lines.iter().take(max_lines).enumerate() {
        let y = area.y.saturating_add(i as u16);
        let style = if hover.language.is_some() {
            Style::default().fg(palette::CYAN)
        } else {
            Style::default().fg(palette::TEXT_PRIMARY)
        };

        let truncated: String = content.chars().take(area.width as usize - 2).collect();
        let span = Span::styled(truncated, style);
        buf.set_span(area.x.saturating_add(1), y, &span, area.width.saturating_sub(2));
    }
}

pub fn render_reference_line(reference: &Reference, selected: bool, area: Rect, buf: &mut Buffer) {
    if area.height == 0 {
        return;
    }

    let bg = if selected { palette::BG_ACTIVE } else { palette::BG_PANEL };
    let style = Style::default().bg(bg);

    for x in area.x..area.x.saturating_add(area.width) {
        buf[(x, area.y)].set_style(style);
    }

    let icon = Span::styled("\u{F0C1}", Style::default().fg(palette::PURPLE));
    let loc = Span::styled(
        format!(" {}:{} ", reference.line, reference.column),
        Style::default().fg(palette::TEXT_MUTED),
    );
    let preview = Span::styled(&reference.preview, Style::default().fg(palette::TEXT_SECONDARY));

    let line = Line::from(vec![icon, loc, preview]);
    buf.set_line(area.x.saturating_add(1), area.y, &line, area.width.saturating_sub(2));
}

pub fn render_panel_tabs(state: &LspState, area: Rect, buf: &mut Buffer) {
    use super::state::LspPanel;

    if area.height == 0 {
        return;
    }

    let tabs = [
        (LspPanel::Diagnostics, "Diagnostics", state.diagnostics.len()),
        (LspPanel::Hover, "Hover", if state.hover.is_some() { 1 } else { 0 }),
        (LspPanel::References, "References", state.references.len()),
    ];

    let mut x = area.x.saturating_add(1);
    for (panel, name, count) in tabs {
        let is_active = state.active_panel == panel;
        let style = if is_active {
            Style::default().fg(palette::CYAN).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette::TEXT_MUTED)
        };

        let text = format!("{} ({}) ", name, count);
        let span = Span::styled(text, style);
        let width = span.width() as u16;
        buf.set_span(x, area.y, &span, width);
        x = x.saturating_add(width);
    }
}
