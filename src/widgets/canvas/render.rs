use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}};
use super::models::{BrushMode, DrawCommand, Point};
use super::state::CanvasState;

const BRAILLE_BASE: char = '\u{2800}';
const BRAILLE_DOTS: [[u8; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

pub fn render_canvas(area: Rect, buf: &mut Buffer, state: &CanvasState) {
    for cmd in &state.commands {
        match cmd {
            DrawCommand::Line { from, to, color } => {
                render_line(area, buf, state, *from, *to, *color);
            }
            DrawCommand::Bezier { p0, p1, p2, p3, color } => {
                render_bezier(area, buf, state, *p0, *p1, *p2, *p3, *color);
            }
            DrawCommand::Dot { at, color } => {
                set_pixel(area, buf, state, at.x as u16, at.y as u16, *color);
            }
        }
    }
}

fn render_line(area: Rect, buf: &mut Buffer, state: &CanvasState, from: Point, to: Point, color: Color) {
    let steps = ((to.x - from.x).abs().max((to.y - from.y).abs()) * 2.0) as usize;
    if steps == 0 { return; }
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let p = from.lerp(to, t);
        set_pixel(area, buf, state, p.x as u16, p.y as u16, color);
    }
}

fn render_bezier(area: Rect, buf: &mut Buffer, state: &CanvasState, p0: Point, p1: Point, p2: Point, p3: Point, color: Color) {
    let steps = 50;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let p = cubic_bezier(p0, p1, p2, p3, t);
        set_pixel(area, buf, state, p.x as u16, p.y as u16, color);
    }
}

fn cubic_bezier(p0: Point, p1: Point, p2: Point, p3: Point, t: f32) -> Point {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;
    Point::new(
        mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x,
        mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y,
    )
}

fn set_pixel(area: Rect, buf: &mut Buffer, state: &CanvasState, x: u16, y: u16, color: Color) {
    let x = x.saturating_sub(state.scroll_x);
    let y = y.saturating_sub(state.scroll_y);
    match state.brush {
        BrushMode::Braille => set_braille_dot(area, buf, x, y, color),
        BrushMode::Block => set_block(area, buf, x, y, color),
        BrushMode::HalfBlock => set_half_block(area, buf, x, y, color),
    }
}

fn set_braille_dot(area: Rect, buf: &mut Buffer, x: u16, y: u16, color: Color) {
    let cell_x = area.x + x / 2;
    let cell_y = area.y + y / 4;
    if cell_x >= area.right() || cell_y >= area.bottom() { return; }
    
    let dot_x = (x % 2) as usize;
    let dot_y = (y % 4) as usize;
    let cell = &mut buf[(cell_x, cell_y)];
    let current = cell.symbol().chars().next().unwrap_or(BRAILLE_BASE);
    let base = if ('\u{2800}'..='\u{28FF}').contains(&current) { 
        current as u32 - BRAILLE_BASE as u32 
    } else { 0 };
    if let Some(ch) = char::from_u32(BRAILLE_BASE as u32 + base + BRAILLE_DOTS[dot_y][dot_x] as u32) {
        cell.set_char(ch).set_style(Style::default().fg(color));
    }
}

fn set_block(area: Rect, buf: &mut Buffer, x: u16, y: u16, color: Color) {
    let cell_x = area.x + x;
    let cell_y = area.y + y;
    if cell_x >= area.right() || cell_y >= area.bottom() { return; }
    buf[(cell_x, cell_y)].set_char('█').set_style(Style::default().fg(color));
}

fn set_half_block(area: Rect, buf: &mut Buffer, x: u16, y: u16, color: Color) {
    let cell_x = area.x + x;
    let cell_y = area.y + y / 2;
    if cell_x >= area.right() || cell_y >= area.bottom() { return; }
    let is_top = y % 2 == 0;
    let ch = if is_top { '▀' } else { '▄' };
    buf[(cell_x, cell_y)].set_char(ch).set_style(Style::default().fg(color));
}
