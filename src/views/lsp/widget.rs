use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::theme::palette;

use super::render::{
    render_diagnostic_line, render_hover_content, render_panel_tabs, render_reference_line,
};
use super::state::{LspPanel, LspState};

pub struct LspView;

impl StatefulWidget for LspView {
    type State = LspState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette::BORDER))
            .title(" LSP ")
            .title_style(Style::default().fg(palette::CYAN));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 {
            return;
        }

        let chunks = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(inner);

        render_panel_tabs(state, chunks[0], buf);

        let content_area = chunks[1];
        match state.active_panel {
            LspPanel::Diagnostics => render_diagnostics_panel(state, content_area, buf),
            LspPanel::Hover => render_hover_panel(state, content_area, buf),
            LspPanel::References => render_references_panel(state, content_area, buf),
        }
    }
}

fn render_diagnostics_panel(state: &LspState, area: Rect, buf: &mut Buffer) {
    if state.diagnostics.is_empty() {
        let msg = "No diagnostics";
        let x = area.x.saturating_add(area.width.saturating_sub(msg.len() as u16) / 2);
        let y = area.y.saturating_add(area.height / 2);
        buf.set_string(x, y, msg, Style::default().fg(palette::TEXT_MUTED));
        return;
    }

    let visible = area.height as usize;
    let start = state.scroll_offset;
    let end = (start + visible).min(state.diagnostics.len());

    for (i, diag) in state.diagnostics[start..end].iter().enumerate() {
        let y = area.y.saturating_add(i as u16);
        let line_area = Rect::new(area.x, y, area.width, 1);
        let selected = start + i == state.selected_diagnostic;
        render_diagnostic_line(diag, selected, line_area, buf);
    }
}

fn render_hover_panel(state: &LspState, area: Rect, buf: &mut Buffer) {
    match &state.hover {
        Some(hover) => render_hover_content(hover, area, buf),
        None => {
            let msg = "No hover information";
            let x = area.x.saturating_add(area.width.saturating_sub(msg.len() as u16) / 2);
            let y = area.y.saturating_add(area.height / 2);
            buf.set_string(x, y, msg, Style::default().fg(palette::TEXT_MUTED));
        }
    }
}

fn render_references_panel(state: &LspState, area: Rect, buf: &mut Buffer) {
    if state.references.is_empty() {
        let msg = "No references";
        let x = area.x.saturating_add(area.width.saturating_sub(msg.len() as u16) / 2);
        let y = area.y.saturating_add(area.height / 2);
        buf.set_string(x, y, msg, Style::default().fg(palette::TEXT_MUTED));
        return;
    }

    let visible = area.height as usize;
    let start = state.scroll_offset;
    let end = (start + visible).min(state.references.len());

    for (i, reference) in state.references[start..end].iter().enumerate() {
        let y = area.y.saturating_add(i as u16);
        let line_area = Rect::new(area.x, y, area.width, 1);
        let selected = start + i == state.selected_reference;
        render_reference_line(reference, selected, line_area, buf);
    }
}
