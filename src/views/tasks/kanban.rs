use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::theme::palette::{BORDER_DIM, BORDER_FOCUS, CYAN, GREEN, MAGENTA, TEXT_PRIMARY};
use super::{list_state::TasksState, render::render_task_line, state::TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KanbanColumn {
    Backlog,
    InProgress,
    Review,
    Done,
}

impl KanbanColumn {
    pub const ALL: [Self; 4] = [Self::Backlog, Self::InProgress, Self::Review, Self::Done];

    pub fn title(&self) -> &'static str {
        match self {
            Self::Backlog => "Backlog",
            Self::InProgress => "In Progress",
            Self::Review => "Review",
            Self::Done => "Done",
        }
    }

    pub fn matches_status(&self, status: TaskStatus) -> bool {
        matches!(
            (self, status),
            (Self::Backlog, TaskStatus::Pending)
                | (Self::InProgress, TaskStatus::InProgress)
                | (Self::Review, TaskStatus::Blocked)
                | (Self::Done, TaskStatus::Done)
        )
    }

    pub fn color(&self) -> ratatui::style::Color {
        match self {
            Self::Backlog => TEXT_PRIMARY,
            Self::InProgress => CYAN,
            Self::Review => MAGENTA,
            Self::Done => GREEN,
        }
    }
}

pub struct KanbanState {
    pub tasks_state: TasksState,
    pub selected_column: usize,
    pub column_scroll: [usize; 4],
    pub column_selected: [usize; 4],
}

impl Default for KanbanState {
    fn default() -> Self {
        Self::new()
    }
}

impl KanbanState {
    pub fn new() -> Self {
        Self {
            tasks_state: TasksState::new(),
            selected_column: 0,
            column_scroll: [0; 4],
            column_selected: [0; 4],
        }
    }

    pub fn column_left(&mut self) {
        self.selected_column = self.selected_column.saturating_sub(1);
    }

    pub fn column_right(&mut self) {
        self.selected_column = (self.selected_column + 1).min(3);
    }

    pub fn select_up(&mut self) {
        let col = self.selected_column;
        self.column_selected[col] = self.column_selected[col].saturating_sub(1);
    }

    pub fn select_down(&mut self) {
        let col = self.selected_column;
        let count = self.column_task_count(col);
        if count > 0 {
            self.column_selected[col] = (self.column_selected[col] + 1).min(count - 1);
        }
    }

    fn column_task_count(&self, col: usize) -> usize {
        let column = KanbanColumn::ALL[col];
        self.tasks_state.tasks.iter().filter(|t| column.matches_status(t.status)).count()
    }
}

pub struct Kanban;

impl StatefulWidget for Kanban {
    type State = KanbanState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let columns = Layout::horizontal([Constraint::Ratio(1, 4); 4]).split(area);

        for (i, (col, col_area)) in KanbanColumn::ALL.iter().zip(columns.iter()).enumerate() {
            let is_focused = i == state.selected_column;
            render_column(buf, *col_area, *col, state, i, is_focused);
        }
    }
}

fn render_column(
    buf: &mut Buffer,
    area: Rect,
    column: KanbanColumn,
    state: &KanbanState,
    col_idx: usize,
    is_focused: bool,
) {
    let border_color = if is_focused { BORDER_FOCUS } else { BORDER_DIM };
    let tasks: Vec<_> = state.tasks_state.tasks.iter()
        .filter(|t| column.matches_status(t.status))
        .collect();

    let title = format!(" {} ({}) ", column.title(), tasks.len());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(title, Style::default().fg(column.color()).add_modifier(Modifier::BOLD)));

    let inner = block.inner(area);
    block.render(area, buf);

    let scroll = state.column_scroll[col_idx];
    let selected = state.column_selected[col_idx];

    for (i, task) in tasks.iter().skip(scroll).take(inner.height as usize).enumerate() {
        let y = inner.y + i as u16;
        let is_selected = is_focused && (scroll + i == selected);
        let line = render_task_line(task, is_selected, inner.width);
        buf.set_line(inner.x, y, &line, inner.width);
    }

    if tasks.is_empty() {
        Paragraph::new(Span::styled("No tasks", Style::default().fg(BORDER_DIM)))
            .render(inner, buf);
    }
}
