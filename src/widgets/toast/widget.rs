use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph, Widget};
use super::render::{action_spans, level_color, progress_line, toast_area, toast_block};
use super::state::ToastState;

pub struct ToastWidget<'a> {
    state: &'a ToastState,
}

impl<'a> ToastWidget<'a> {
    pub fn new(state: &'a ToastState) -> Self {
        Self { state }
    }

    pub fn render_overlay(self, frame: &mut ratatui::Frame) {
        let Some(toast) = self.state.active() else { return };
        
        let area = toast_area(frame.area(), toast);
        let block = toast_block(toast);
        let inner = block.inner(area);
        
        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
        
        let icon = toast.level.icon();
        let msg = Line::from(vec![
            Span::styled(format!("{} ", icon), Style::default().fg(level_color(toast.level))),
            Span::raw(&toast.message),
        ]);
        frame.render_widget(Paragraph::new(msg), inner);
        
        if inner.height > 1 {
            let progress_area = Rect::new(inner.x, inner.y + 1, inner.width, 1);
            let progress = progress_line(toast, inner.width);
            frame.render_widget(Paragraph::new(progress), progress_area);
        }
        
        if inner.height > 2 && (!toast.actions.is_empty() || toast.dismissible) {
            let actions_area = Rect::new(inner.x, inner.y + 2, inner.width, 1);
            let actions = action_spans(toast, self.state.selected_index());
            frame.render_widget(Paragraph::new(actions), actions_area);
        }
    }
}

impl Widget for ToastWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(toast) = self.state.active() else { return };
        
        let toast_area = toast_area(area, toast);
        if toast_area.x >= area.width { return; }
        
        let block = toast_block(toast);
        let inner = block.inner(toast_area);
        block.render(toast_area, buf);
        
        let icon = toast.level.icon();
        let color = level_color(toast.level);
        if inner.width > 0 && inner.height > 0 {
            buf.set_string(inner.x, inner.y, icon, Style::default().fg(color));
            let msg_x = inner.x + 2;
            let msg_width = inner.width.saturating_sub(2) as usize;
            let msg: String = toast.message.chars().take(msg_width).collect();
            buf.set_string(msg_x, inner.y, &msg, Style::default());
        }
    }
}
