use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, StatefulWidget, Widget},
};

use super::list_state::BufferListState;

pub struct BufferListWidget<'a> {
    block: Option<Block<'a>>,
    highlight_style: Style,
}

impl<'a> Default for BufferListWidget<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> BufferListWidget<'a> {
    pub fn new() -> Self {
        Self {
            block: None,
            highlight_style: Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl StatefulWidget for BufferListWidget<'_> {
    type State = BufferListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let inner = match self.block {
            Some(b) => {
                let inner = b.inner(area);
                b.render(area, buf);
                inner
            }
            None => area,
        };

        if inner.height == 0 || inner.width == 0 {
            return;
        }

        state.adjust_scroll(inner.height as usize);

        let visible_buffers = state
            .buffers
            .iter()
            .enumerate()
            .skip(state.scroll_offset)
            .take(inner.height as usize);

        for (idx, (i, buffer)) in visible_buffers.enumerate() {
            let y = inner.y + idx as u16;
            let is_selected = i == state.selected_index;

            let modified_indicator = if buffer.is_modified { " \u{F0415}" } else { "" };
            let name = buffer.name.rsplit('/').next().unwrap_or(&buffer.name);
            let line_info = format!(" ({} lines)", buffer.line_count);

            let style = if is_selected {
                self.highlight_style
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::styled(format!(" {:>3} ", buffer.id), style.fg(Color::DarkGray)),
                Span::styled(name, style),
                Span::styled(modified_indicator, style.fg(Color::Yellow)),
                Span::styled(line_info, style.fg(Color::DarkGray)),
            ]);

            let line_area = Rect::new(inner.x, y, inner.width, 1);
            if is_selected {
                buf.set_style(line_area, self.highlight_style);
            }
            line.render(line_area, buf);
        }

        if state.is_empty() {
            let msg = "No buffers";
            let x = inner.x + (inner.width.saturating_sub(msg.len() as u16)) / 2;
            let y = inner.y + inner.height / 2;
            buf.set_string(x, y, msg, Style::default().fg(Color::DarkGray));
        }
    }
}
