use ratatui::style::Color;

use crate::theme::colors::palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn day_of_week(&self) -> u8 {
        let y = if self.month < 3 {
            self.year as i32 - 1
        } else {
            self.year as i32
        };
        let m = if self.month < 3 {
            self.month as i32 + 12
        } else {
            self.month as i32
        };
        let d = self.day as i32;
        let dow = (d + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
        ((dow + 6) % 7) as u8
    }

    pub fn week_of_year(&self) -> u8 {
        let first_day = Date::new(self.year, 1, 1);
        let days_since = self.days_since_year_start();
        let first_dow = first_day.day_of_week();
        ((days_since + first_dow as u16) / 7) as u8
    }

    fn days_since_year_start(&self) -> u16 {
        let days_in_months = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut days = days_in_months[self.month as usize - 1] + self.day as u16 - 1;
        if self.month > 2 && self.is_leap_year() {
            days += 1;
        }
        days
    }

    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }
}

#[derive(Debug, Clone)]
pub struct HeatmapEntry {
    pub date: Date,
    pub value: u32,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct HeatmapConfig {
    pub show_month_labels: bool,
    pub show_day_labels: bool,
    pub cell_char: char,
    pub empty_char: char,
}

impl Default for HeatmapConfig {
    fn default() -> Self {
        Self {
            show_month_labels: true,
            show_day_labels: true,
            cell_char: '█',
            empty_char: '░',
        }
    }
}

pub fn intensity_color(value: u32, max_value: u32) -> Color {
    if value == 0 {
        return palette::BG_SURFACE;
    }
    let ratio = value as f32 / max_value.max(1) as f32;
    match ratio {
        r if r < 0.25 => palette::GREEN_DIM,
        r if r < 0.50 => palette::GREEN,
        r if r < 0.75 => palette::CYAN,
        _ => palette::CYAN,
    }
}
