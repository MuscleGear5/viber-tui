use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use super::{models::SparklineStyle, state::SparklineState};

const BRAILLE_BLOCKS: [char; 8] = ['⣀', '⣤', '⣶', '⣿', '⡇', '⡏', '⠛', '⠉'];
const BAR_BLOCKS: [&str; 8] = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
const DOT_CHAR: &str = "•";

pub fn render_sparkline(state: &mut SparklineState, area: Rect, buf: &mut Buffer) {
    if area.width == 0 || area.height == 0 || state.data.is_empty() {
        return;
    }

    let style = Style::default().fg(state.config.color);
    let (min, max) = match state.range() {
        Some((min, max)) if (max - min).abs() > f64::EPSILON => (min, max),
        Some((val, _)) => (val - 1.0, val + 1.0),
        None => return,
    };

    let baseline = state.config.baseline.unwrap_or(min);
    let range = max - baseline.min(min);

    let width = area.width as usize;
    let data_len = state.data.len();
    let start_idx = data_len.saturating_sub(width);
    let points = &state.data[start_idx..];

    match state.config.style {
        SparklineStyle::Line => render_line(points, min, range, area, buf, style),
        SparklineStyle::Bar => render_bars(points, min, range, area, buf, style),
        SparklineStyle::Dot => render_dots(points, min, range, area, buf, style),
    }
}

fn render_line(points: &[super::models::DataPoint], min: f64, range: f64, area: Rect, buf: &mut Buffer, style: Style) {
    for (i, point) in points.iter().enumerate() {
        if i >= area.width as usize {
            break;
        }
        let normalized = ((point.value - min) / range).clamp(0.0, 1.0);
        let level = (normalized * 7.0) as usize;
        let x = area.x + i as u16;
        let y = area.y + area.height - 1;
        buf[(x, y)].set_char(BRAILLE_BLOCKS[level]).set_style(style);
    }
}

fn render_bars(points: &[super::models::DataPoint], min: f64, range: f64, area: Rect, buf: &mut Buffer, style: Style) {
    for (i, point) in points.iter().enumerate() {
        if i >= area.width as usize {
            break;
        }
        let normalized = ((point.value - min) / range).clamp(0.0, 1.0);
        let level = (normalized * 7.0) as usize;
        let x = area.x + i as u16;
        let y = area.y + area.height - 1;
        buf[(x, y)].set_symbol(BAR_BLOCKS[level]).set_style(style);
    }
}

fn render_dots(points: &[super::models::DataPoint], min: f64, range: f64, area: Rect, buf: &mut Buffer, style: Style) {
    for (i, point) in points.iter().enumerate() {
        if i >= area.width as usize {
            break;
        }
        let normalized = ((point.value - min) / range).clamp(0.0, 1.0);
        let y_offset = ((1.0 - normalized) * (area.height - 1) as f64) as u16;
        let x = area.x + i as u16;
        let y = area.y + y_offset;
        buf[(x, y)].set_symbol(DOT_CHAR).set_style(style);
    }
}
