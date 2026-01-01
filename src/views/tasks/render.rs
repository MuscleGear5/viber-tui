use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::theme::palette::{BG_HIGHLIGHT, TEXT_PRIMARY, TEXT_SECONDARY};
use super::state::Task;

pub fn render_task_line(task: &Task, is_selected: bool, width: u16) -> Line<'static> {
    let bg = if is_selected { BG_HIGHLIGHT } else { ratatui::style::Color::Reset };
    let base_style = Style::default().bg(bg);

    let status_span = Span::styled(
        format!("{} ", task.status.icon()),
        Style::default().fg(task.status.color()).bg(bg),
    );

    let priority_span = Span::styled(
        format!("{} ", task.priority.icon()),
        Style::default().fg(task.priority.color()).bg(bg),
    );

    let title_style = if is_selected {
        base_style.fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD)
    } else {
        base_style.fg(TEXT_PRIMARY)
    };

    let available_width = width.saturating_sub(8) as usize;
    let title = if task.title.len() > available_width {
        format!("{}…", &task.title[..available_width.saturating_sub(1)])
    } else {
        task.title.clone()
    };

    let title_span = Span::styled(title, title_style);

    let tag_str = if !task.tags.is_empty() {
        format!(" [{}]", task.tags.join(", "))
    } else {
        String::new()
    };
    let tags_span = Span::styled(tag_str, Style::default().fg(TEXT_SECONDARY).bg(bg));

    Line::from(vec![status_span, priority_span, title_span, tags_span])
}

pub fn render_task_count(total: usize, filtered: usize) -> Line<'static> {
    let count_str = if total == filtered {
        format!("{} tasks", total)
    } else {
        format!("{}/{} tasks", filtered, total)
    };

    Line::from(Span::styled(count_str, Style::default().fg(TEXT_SECONDARY)))
}
