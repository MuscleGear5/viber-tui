use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding};
use super::models::{ToastLevel, ToastNotification};
use crate::theme::colors::palette;

pub fn toast_area(frame_area: Rect, toast: &ToastNotification) -> Rect {
    let width = 50.min(frame_area.width.saturating_sub(4));
    let height = 3 + if toast.actions.is_empty() { 0 } else { 1 };
    
    let slide = toast.slide_progress();
    let target_x = frame_area.width.saturating_sub(width + 2);
    let start_x = frame_area.width;
    let x = start_x - ((start_x - target_x) as f32 * slide) as u16;
    
    Rect::new(x, 1, width, height)
}

pub fn level_color(level: ToastLevel) -> Color {
    match level {
        ToastLevel::Info => palette::INFO,
        ToastLevel::Success => palette::SUCCESS,
        ToastLevel::Warning => palette::WARNING,
        ToastLevel::Error => palette::ERROR,
    }
}

pub fn toast_block(toast: &ToastNotification) -> Block<'static> {
    let color = level_color(toast.level);
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .padding(Padding::horizontal(1))
}

pub fn progress_line(toast: &ToastNotification, width: u16) -> Line<'static> {
    if toast.duration_ms == u64::MAX {
        return Line::default();
    }
    
    let progress = toast.progress();
    let filled = ((width as f32) * (1.0 - progress)) as usize;
    let empty = (width as usize).saturating_sub(filled);
    let color = level_color(toast.level);
    
    Line::from(vec![
        Span::styled("\u{2593}".repeat(filled), Style::default().fg(color)),
        Span::styled("\u{2591}".repeat(empty), Style::default().fg(palette::TEXT_MUTED)),
    ])
}

pub fn action_spans(toast: &ToastNotification, selected: usize) -> Line<'static> {
    let mut spans = Vec::new();
    let color = level_color(toast.level);
    
    for (i, action) in toast.actions.iter().enumerate() {
        if i > 0 { spans.push(Span::raw(" ")); }
        let style = if i == selected {
            Style::default().fg(palette::BG_VOID).bg(color).bold()
        } else {
            Style::default().fg(color)
        };
        spans.push(Span::styled(format!("[{}]", action.label), style));
    }
    
    if toast.dismissible {
        if !spans.is_empty() { spans.push(Span::raw(" ")); }
        let dismiss_idx = toast.actions.len();
        let style = if selected == dismiss_idx {
            Style::default().fg(palette::BG_VOID).bg(palette::TEXT_MUTED).bold()
        } else {
            Style::default().fg(palette::TEXT_MUTED)
        };
        spans.push(Span::styled("[Dismiss]", style));
    }
    
    Line::from(spans)
}
