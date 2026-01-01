use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::theme::palette::{BORDER_FOCUS, CYAN, GREEN, MAGENTA, TEXT_SECONDARY};
use super::{render::{render_phase_line, render_progress_bar}, state::WorkflowState};

pub struct Workflow<'a> {
    title: &'a str,
}

impl<'a> Default for Workflow<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Workflow<'a> {
    pub fn new() -> Self {
        Self { title: "Workflow" }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
}

impl StatefulWidget for Workflow<'_> {
    type State = WorkflowState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER_FOCUS))
            .title(Span::styled(
                format!(" {} ", self.title),
                Style::default().fg(MAGENTA).add_modifier(Modifier::BOLD),
            ));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 {
            return;
        }

        let chunks = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(3),
        ]).split(inner);

        render_header(buf, chunks[0], state);
        render_phases(buf, chunks[1], state);
    }
}

fn render_header(buf: &mut Buffer, area: Rect, state: &WorkflowState) {
    let progress = state.overall_progress();
    let header = Line::from(vec![
        Span::styled("\u{F0E6F} ", Style::default().fg(GREEN)),
        Span::styled(
            format!("Overall: {}%  ", progress),
            Style::default().fg(TEXT_SECONDARY),
        ),
    ]);
    
    Paragraph::new(header).render(area, buf);
    
    if area.width > 20 {
        let bar_area = Rect::new(area.x + 16, area.y, area.width.saturating_sub(17), 1);
        render_progress_bar(buf, bar_area, progress);
    }
}

fn render_phases(buf: &mut Buffer, area: Rect, state: &mut WorkflowState) {
    let visible_height = area.height as usize;
    
    if state.selected_index >= state.scroll_offset + visible_height {
        state.scroll_offset = state.selected_index.saturating_sub(visible_height - 1);
    } else if state.selected_index < state.scroll_offset {
        state.scroll_offset = state.selected_index;
    }

    for (i, phase) in state.phases.iter().skip(state.scroll_offset).take(visible_height).enumerate() {
        let y = area.y + i as u16;
        if y >= area.y + area.height {
            break;
        }

        let is_selected = state.scroll_offset + i == state.selected_index;
        let line = render_phase_line(phase, is_selected, area.width);
        
        buf.set_line(area.x, y, &line, area.width);
    }
}
