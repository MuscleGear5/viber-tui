use super::models::{BrushMode, DrawCommand};

#[derive(Debug, Default)]
pub struct CanvasState {
    pub commands: Vec<DrawCommand>,
    pub brush: BrushMode,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl CanvasState {
    pub fn new() -> Self { Self::default() }
    
    pub fn with_brush(mut self, brush: BrushMode) -> Self {
        self.brush = brush;
        self
    }
    
    pub fn push(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }
    
    pub fn clear(&mut self) {
        self.commands.clear();
    }
    
    pub fn scroll(&mut self, dx: i16, dy: i16) {
        self.scroll_x = (self.scroll_x as i16 + dx).max(0) as u16;
        self.scroll_y = (self.scroll_y as i16 + dy).max(0) as u16;
    }
}
