use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::theme::palette::{BORDER_FOCUS, CYAN, TEXT_SECONDARY};
use super::{render::{render_task_count, render_task_line}, list_state::TasksState};

pub struct Tasks<'a> {
    title: &'a str,
    show_filter: bool,
}

impl<'a> Default for Tasks<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Tasks<'a> {
    pub fn new() -> Self {
        Self { 
            title: "Tasks",
            show_filter: true,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn show_filter(mut self, show: bool) -> Self {
        self.show_filter = show;
        self
    }
}

impl StatefulWidget for Tasks<'_> {
    type State = TasksState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER_FOCUS))
            .title(Span::styled(
                format!(" {} ", self.title),
                Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
            ));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 {
            return;
        }

        let chunks = if self.show_filter {
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
            ]).split(inner)
        } else {
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Min(1),
            ]).split(inner)
        };

        let (filter_area, count_area, list_area) = if self.show_filter {
            (Some(chunks[0]), chunks[1], chunks[2])
        } else {
            (None, chunks[0], chunks[1])
        };

        if let Some(fa) = filter_area {
            render_filter_bar(buf, fa, &state.filter_query);
        }

        let count_line = render_task_count(state.tasks.len(), state.filtered_indices.len());
        Paragraph::new(count_line).render(count_area, buf);

        render_task_list(buf, list_area, state);
    }
}

fn render_filter_bar(buf: &mut Buffer, area: Rect, query: &str) {
    let filter_text = if query.is_empty() {
        Span::styled("/ to filter...", Style::default().fg(TEXT_SECONDARY))
    } else {
        Span::styled(format!("Filter: {}", query), Style::default().fg(CYAN))
    };
    
    Paragraph::new(filter_text).render(area, buf);
}

fn render_task_list(buf: &mut Buffer, area: Rect, state: &mut TasksState) {
    let visible_height = area.height as usize;

    if state.selected_index >= state.scroll_offset + visible_height {
        state.scroll_offset = state.selected_index.saturating_sub(visible_height - 1);
    } else if state.selected_index < state.scroll_offset {
        state.scroll_offset = state.selected_index;
    }

    for (i, &task_idx) in state.filtered_indices.iter()
        .skip(state.scroll_offset)
        .take(visible_height)
        .enumerate() 
    {
        let y = area.y + i as u16;
        if y >= area.y + area.height {
            break;
        }

        if let Some(task) = state.tasks.get(task_idx) {
            let is_selected = state.scroll_offset + i == state.selected_index;
            let line = render_task_line(task, is_selected, area.width);
            buf.set_line(area.x, y, &line, area.width);
        }
    }
}
