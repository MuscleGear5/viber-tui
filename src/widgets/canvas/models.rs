use ratatui::style::Color;

#[derive(Debug, Clone, Copy, Default)]
pub enum BrushMode {
    #[default]
    Braille,
    Block,
    HalfBlock,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

impl From<(u16, u16)> for Point {
    fn from((x, y): (u16, u16)) -> Self { Self::new(x as f32, y as f32) }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self { Self::new(x, y) }
}

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Line { from: Point, to: Point, color: Color },
    Bezier { p0: Point, p1: Point, p2: Point, p3: Point, color: Color },
    Dot { at: Point, color: Color },
}

impl DrawCommand {
    pub fn line(from: impl Into<Point>, to: impl Into<Point>, color: Color) -> Self {
        Self::Line { from: from.into(), to: to.into(), color }
    }
    
    pub fn bezier(p0: impl Into<Point>, p1: impl Into<Point>, p2: impl Into<Point>, p3: impl Into<Point>, color: Color) -> Self {
        Self::Bezier { p0: p0.into(), p1: p1.into(), p2: p2.into(), p3: p3.into(), color }
    }
    
    pub fn dot(at: impl Into<Point>, color: Color) -> Self {
        Self::Dot { at: at.into(), color }
    }
}
