use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
};

use crate::theme::colors::palette;

use super::{
    models::{intensity_color, Date},
    state::HeatmapState,
};

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const DAYS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];

pub fn render_heatmap(area: Rect, buf: &mut Buffer, state: &HeatmapState) {
    if area.width < 10 || area.height < 3 {
        return;
    }

    let cfg = &state.config;
    let x_offset = if cfg.show_day_labels { 2 } else { 0 };
    let y_offset = if cfg.show_month_labels { 1 } else { 0 };

    if cfg.show_day_labels && area.height > y_offset + 7 {
        for (i, day) in DAYS.iter().enumerate() {
            let y = area.y + y_offset + i as u16;
            if y < area.y + area.height {
                buf.set_string(area.x, y, day, Style::default().fg(palette::TEXT_MUTED));
            }
        }
    }

    if cfg.show_month_labels {
        let mut month = 1u8;
        let mut last_month_x = 0u16;
        for week in 0..53u8 {
            let x = area.x + x_offset + week as u16;
            if x >= area.x + area.width {
                break;
            }
            let date = date_from_week_day(state.year(), week, 0);
            if date.month != month && x > last_month_x + 3 {
                buf.set_string(x, area.y, MONTHS[date.month as usize - 1], 
                    Style::default().fg(palette::TEXT_MUTED));
                month = date.month;
                last_month_x = x;
            }
        }
    }

    let selected = state.selected();
    for week in 0..53u8 {
        let x = area.x + x_offset + week as u16;
        if x >= area.x + area.width {
            break;
        }
        for day in 0..7u8 {
            let y = area.y + y_offset + day as u16;
            if y >= area.y + area.height {
                break;
            }
            let date = date_from_week_day(state.year(), week, day);
            let value = state.get_entry(&date).map(|e| e.value).unwrap_or(0);
            let ch = if value > 0 { cfg.cell_char } else { cfg.empty_char };
            let mut color = intensity_color(value, state.max_value());
            if selected == Some((week, day)) {
                color = palette::PINK;
            }
            buf.set_string(x, y, ch.to_string(), Style::default().fg(color));
        }
    }
}

fn date_from_week_day(year: u16, week: u8, day: u8) -> Date {
    let jan1 = Date::new(year, 1, 1);
    let jan1_dow = jan1.day_of_week();
    let days_offset = week as i16 * 7 + day as i16 - jan1_dow as i16;
    let day_of_year = days_offset.max(0) as u16;
    day_of_year_to_date(year, day_of_year)
}

fn day_of_year_to_date(year: u16, doy: u16) -> Date {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days: [u8; 12] = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let (mut remaining, mut month) = (doy, 0usize);
    while month < 12 && remaining >= days[month] as u16 {
        remaining -= days[month] as u16;
        month += 1;
    }
    Date::new(year, (month + 1).min(12) as u8, (remaining + 1).min(31) as u8)
}
