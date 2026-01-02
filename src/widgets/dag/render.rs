use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use crate::theme::palette;
use super::state::DagState;

const BRAILLE_BASE: char = '\u{2800}';
const BRAILLE_DOTS: [[u8; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

pub fn render_dag(area: Rect, buf: &mut Buffer, state: &DagState) {
    let block_style = Style::default().bg(palette::BG_PANEL);
    buf.set_style(area, block_style);

    for edge in state.edges() {
        render_edge_braille(area, buf, state, edge.from_x, edge.from_y, edge.to_x, edge.to_y);
    }
    for node in state.nodes.values() {
        render_node(area, buf, state, node, state.selected.as_ref() == Some(&node.id));
    }
}

fn render_node(area: Rect, buf: &mut Buffer, state: &DagState, node: &super::models::DagNode, selected: bool) {
    let x = node.x.saturating_sub(state.scroll_x);
    let y = node.y.saturating_sub(state.scroll_y);
    if x >= area.width || y >= area.height { return; }

    let abs_x = area.x + x;
    let abs_y = area.y + y;
    let style = Style::default().fg(node.status.color());
    let border = if selected { Style::default().fg(palette::CYAN) } else { style };

    if abs_x < area.right() { buf[(abs_x, abs_y)].set_char(node.status.icon().chars().next().unwrap_or('●')).set_style(style); }
    let label: String = node.label.chars().take((area.width - x - 2) as usize).collect();
    for (i, ch) in label.chars().enumerate() {
        let lx = abs_x + 2 + i as u16;
        if lx < area.right() { buf[(lx, abs_y)].set_char(ch).set_style(border); }
    }
}

fn render_edge_braille(area: Rect, buf: &mut Buffer, state: &DagState, x1: u16, y1: u16, x2: u16, y2: u16) {
    let sx = state.scroll_x; let sy = state.scroll_y;
    let (x1, y1) = (x1.saturating_sub(sx), y1.saturating_sub(sy));
    let (x2, y2) = (x2.saturating_sub(sx), y2.saturating_sub(sy));

    let steps = ((x2 as i32 - x1 as i32).abs().max((y2 as i32 - y1 as i32).abs()) * 2) as usize;
    if steps == 0 { return; }
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let px = (x1 as f32 + (x2 as f32 - x1 as f32) * t) as u16;
        let py = (y1 as f32 + (y2 as f32 - y1 as f32) * t) as u16;
        set_braille_dot(area, buf, px, py);
    }
}

fn set_braille_dot(area: Rect, buf: &mut Buffer, x: u16, y: u16) {
    let cell_x = area.x + x / 2;
    let cell_y = area.y + y / 4;
    if cell_x >= area.right() || cell_y >= area.bottom() { return; }

    let dot_x = (x % 2) as usize;
    let dot_y = (y % 4) as usize;
    let cell = &mut buf[(cell_x, cell_y)];
    let current = cell.symbol().chars().next().unwrap_or(BRAILLE_BASE);
    let base = if current >= BRAILLE_BASE && current <= '\u{28FF}' { current as u32 - BRAILLE_BASE as u32 } else { 0 };
    let new_char = char::from_u32(BRAILLE_BASE as u32 + base + BRAILLE_DOTS[dot_y][dot_x] as u32).unwrap_or(BRAILLE_BASE);
    cell.set_char(new_char).set_style(Style::default().fg(palette::BORDER_SUBTLE));
}
