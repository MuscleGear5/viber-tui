use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::theme::{animation::AnimationState, palette};

use super::render::{message_height, render_message};
use super::state::ChatState;

pub struct Chat<'a> {
    animation: &'a AnimationState,
}

impl<'a> Chat<'a> {
    pub fn new(animation: &'a AnimationState) -> Self {
        Self { animation }
    }

    fn pulse_border(&self, base: ratatui::style::Color) -> ratatui::style::Color {
        let intensity = self.animation.pulse();
        match base {
            ratatui::style::Color::Rgb(r, g, b) => {
                let boost = (intensity * 40.0) as u8;
                ratatui::style::Color::Rgb(
                    r.saturating_add(boost),
                    g.saturating_add(boost),
                    b.saturating_add(boost),
                )
            }
            _ => base,
        }
    }
}

impl<'a> StatefulWidget for Chat<'a> {
    type State = ChatState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(area);

        render_messages_area(&self, chunks[0], buf, state);
        render_input_area(&self, chunks[1], buf, state);
    }
}

fn render_messages_area(_chat: &Chat, area: Rect, buf: &mut Buffer, state: &ChatState) {
    let block = Block::default()
        .title("\u{F4AD} Chat")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette::BORDER));

    let inner = block.inner(area);
    block.render(area, buf);

    if state.messages.is_empty() {
        let placeholder = Paragraph::new("No messages yet...")
            .style(Style::default().fg(palette::TEXT_DIM));
        placeholder.render(inner, buf);
        return;
    }

    let mut y = inner.y;
    let visible_start = state.scroll_offset;

    for msg in state.messages.iter().skip(visible_start) {
        if y >= inner.y + inner.height {
            break;
        }

        let msg_height = message_height(msg, inner.width).min(inner.height - (y - inner.y));
        let msg_area = Rect::new(inner.x, y, inner.width, msg_height);

        render_message(msg, msg_area, buf);
        y += msg_height;
    }
}

fn render_input_area(chat: &Chat, area: Rect, buf: &mut Buffer, state: &ChatState) {
    let border_color = if state.input_focused {
        chat.pulse_border(palette::CYAN)
    } else {
        palette::BORDER
    };

    let block = Block::default()
        .title("\u{F11C} Input")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    block.render(area, buf);

    if state.input.is_empty() && !state.input_focused {
        let placeholder = Paragraph::new("Type a message...")
            .style(Style::default().fg(palette::TEXT_DIM));
        placeholder.render(inner, buf);
        return;
    }

    let input_style = Style::default().fg(palette::TEXT_PRIMARY);
    let cursor_visible = chat.animation.cursor_visible() && state.input_focused;

    for (i, ch) in state.input.chars().enumerate() {
        let x = inner.x + i as u16;
        if x >= inner.x + inner.width {
            break;
        }
        let style = if cursor_visible && i == state.cursor_pos {
            input_style.bg(palette::CYAN).fg(palette::BG_VOID)
        } else {
            input_style
        };
        buf.set_string(x, inner.y, ch.to_string(), style);
    }

    if cursor_visible && state.cursor_pos >= state.input.len() {
        let x = inner.x + state.input.len() as u16;
        if x < inner.x + inner.width {
            buf.set_string(x, inner.y, " ", input_style.bg(palette::CYAN));
        }
    }
}
