use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph, Widget, Wrap};
use super::render::{button_spans, input_line, modal_area, modal_block};
use super::state::ModalState;

pub struct ModalWidget<'a> {
    state: &'a ModalState,
}

impl<'a> ModalWidget<'a> {
    pub fn new(state: &'a ModalState) -> Self {
        Self { state }
    }

    pub fn render_overlay(self, frame: &mut ratatui::Frame) {
        let Some(modal) = self.state.active() else { return };
        
        let area = modal_area(frame.area(), modal);
        let block = modal_block(modal);
        let inner = block.inner(area);
        
        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
        
        let msg = Paragraph::new(modal.message.clone())
            .wrap(Wrap { trim: true });
        frame.render_widget(msg, inner);
        
        let msg_lines = modal.message.lines().count() as u16;
        let mut y_offset = msg_lines + 1;
        
        if let Some(ref value) = modal.input_value {
            let input_area = Rect::new(inner.x, inner.y + y_offset, inner.width, 1);
            let placeholder = modal.input_placeholder.as_deref();
            let input = input_line(value, placeholder, inner.width);
            frame.render_widget(Paragraph::new(input), input_area);
            y_offset += 2;
        }
        
        if !modal.buttons.is_empty() {
            let btn_area = Rect::new(inner.x, inner.y + y_offset, inner.width, 1);
            let buttons = button_spans(modal, self.state.selected_button());
            frame.render_widget(Paragraph::new(buttons), btn_area);
        }
    }
}

impl Widget for ModalWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(modal) = self.state.active() else { return };
        
        let modal_area = modal_area(area, modal);
        let block = modal_block(modal);
        let inner = block.inner(modal_area);
        
        for y in modal_area.y..modal_area.y + modal_area.height {
            for x in modal_area.x..modal_area.x + modal_area.width {
                if x < buf.area.width && y < buf.area.height {
                    buf[(x, y)].reset();
                }
            }
        }
        
        block.render(modal_area, buf);
        
        if inner.width > 0 && inner.height > 0 {
            let msg: String = modal.message.chars().take(inner.width as usize).collect();
            buf.set_string(inner.x, inner.y, &msg, Style::default());
        }
    }
}
