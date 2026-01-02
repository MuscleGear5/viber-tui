use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use super::state::CanvasState;
use super::render::render_canvas;

pub struct CanvasWidget<'a> {
    state: &'a CanvasState,
}

impl<'a> CanvasWidget<'a> {
    pub fn new(state: &'a CanvasState) -> Self {
        Self { state }
    }
}

impl Widget for CanvasWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_canvas(area, buf, self.state);
    }
}
