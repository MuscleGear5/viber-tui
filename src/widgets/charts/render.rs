use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

use super::{ChartKind, ChartState};

const BRAILLE_BASE: char = '\u{2800}';
const BRAILLE_DOTS: [[u8; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

pub fn render_chart(area: Rect, buf: &mut Buffer, state: &ChartState) {
    if area.width < 4 || area.height < 3 {
        return;
    }

    let chart_area = compute_chart_area(area, state);

    if state.config.show_y_axis {
        render_y_axis(area, buf, state);
    }

    if state.config.show_x_axis && area.height > 2 {
        render_x_axis(chart_area, buf, state);
    }

    match state.config.kind {
        ChartKind::Line => render_line_chart(chart_area, buf, state),
        ChartKind::Bar => render_bar_chart(chart_area, buf, state),
    }

    if state.config.show_legend && !state.series.is_empty() {
        render_legend(area, buf, state);
    }
}

fn compute_chart_area(area: Rect, state: &ChartState) -> Rect {
    let x_offset = if state.config.show_y_axis { 6 } else { 0 };
    let height_offset = if state.config.show_x_axis { 1 } else { 0 };

    Rect {
        x: area.x + x_offset,
        y: area.y,
        width: area.width.saturating_sub(x_offset),
        height: area.height.saturating_sub(height_offset),
    }
}

fn render_y_axis(area: Rect, buf: &mut Buffer, state: &ChartState) {
    let (min, max) = state.y_range();
    let style = Style::default().fg(Color::DarkGray);

    for row in 0..area.height.saturating_sub(1) {
        let ratio = 1.0 - (row as f64 / (area.height.saturating_sub(2)) as f64);
        let value = min + ratio * (max - min);
        let label = format!("{:>5.0}", value);
        buf.set_string(area.x, area.y + row, &label, style);
    }
}

fn render_x_axis(chart_area: Rect, buf: &mut Buffer, state: &ChartState) {
    let y = chart_area.y + chart_area.height;
    let style = Style::default().fg(Color::DarkGray);

    for (i, label) in state.config.x_labels.iter().enumerate() {
        let x = chart_area.x + (i as u16 * 8).min(chart_area.width.saturating_sub(1));
        if x < chart_area.x + chart_area.width {
            buf.set_string(x, y, label, style);
        }
    }
}

fn render_line_chart(area: Rect, buf: &mut Buffer, state: &ChartState) {
    let (y_min, y_max) = state.y_range();
    let y_range = (y_max - y_min).max(1.0);
    let height_cells = area.height as usize;

    for series in &state.series {
        if series.data.is_empty() {
            continue;
        }

        let style = Style::default().fg(series.color);
        let points_per_col = (series.data.len() as f64 / area.width as f64).max(1.0);

        for col in 0..area.width as usize {
            let data_idx = (col as f64 * points_per_col) as usize;
            if data_idx >= series.data.len() {
                break;
            }

            let value = series.data[data_idx];
            let normalized = ((value - y_min) / y_range).clamp(0.0, 1.0);
            let y_pos = ((1.0 - normalized) * (height_cells.saturating_sub(1)) as f64) as u16;

            let cell_x = area.x + col as u16;
            let cell_y = area.y + y_pos;

            if cell_y < area.y + area.height {
                let dot = braille_dot(normalized, height_cells);
                buf.set_string(cell_x, cell_y, &dot.to_string(), style);
            }
        }
    }
}

fn render_bar_chart(area: Rect, buf: &mut Buffer, state: &ChartState) {
    let (y_min, y_max) = state.y_range();
    let y_range = (y_max - y_min).max(1.0);
    let num_series = state.series.len();

    if num_series == 0 {
        return;
    }

    let max_len = state.max_data_len();
    let group_width = (area.width as usize / max_len.max(1)).max(1);
    let bar_width = (group_width / num_series).max(1);

    for (si, series) in state.series.iter().enumerate() {
        let style = Style::default().fg(series.color);

        for (di, &value) in series.data.iter().enumerate() {
            let normalized = ((value - y_min) / y_range).clamp(0.0, 1.0);
            let bar_height = (normalized * area.height as f64) as u16;

            let x = area.x + (di * group_width + si * bar_width) as u16;
            if x >= area.x + area.width {
                break;
            }

            for row in 0..bar_height {
                let y = area.y + area.height - 1 - row;
                buf.set_string(x, y, "\u{2588}", style);
            }
        }
    }
}

fn render_legend(area: Rect, buf: &mut Buffer, state: &ChartState) {
    let mut x = area.x + area.width.saturating_sub(20);

    for series in &state.series {
        let style = Style::default().fg(series.color);
        buf.set_string(x, area.y, "\u{25A0}", style);
        buf.set_string(x + 2, area.y, &series.name, Style::default());
        x += (series.name.len() as u16) + 4;
    }
}

fn braille_dot(normalized: f64, _height: usize) -> char {
    let row = ((1.0 - normalized) * 3.0) as usize;
    let dot_pattern = BRAILLE_DOTS[row.min(3)][0];
    char::from_u32(BRAILLE_BASE as u32 + dot_pattern as u32).unwrap_or('·')
}
