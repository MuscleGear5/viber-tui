use std::collections::HashMap;

use super::models::{Date, HeatmapConfig, HeatmapEntry};

#[derive(Debug, Clone)]
pub struct HeatmapState {
    entries: HashMap<(u16, u8, u8), HeatmapEntry>,
    year: u16,
    max_value: u32,
    selected_week: Option<u8>,
    selected_day: Option<u8>,
    pub config: HeatmapConfig,
}

impl HeatmapState {
    pub fn new(year: u16) -> Self {
        Self {
            entries: HashMap::new(),
            year,
            max_value: 0,
            selected_week: None,
            selected_day: None,
            config: HeatmapConfig::default(),
        }
    }

    pub fn add_entry(&mut self, entry: HeatmapEntry) {
        if entry.value > self.max_value {
            self.max_value = entry.value;
        }
        let key = (entry.date.year, entry.date.month, entry.date.day);
        self.entries.insert(key, entry);
    }

    pub fn get_entry(&self, date: &Date) -> Option<&HeatmapEntry> {
        self.entries.get(&(date.year, date.month, date.day))
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn max_value(&self) -> u32 {
        self.max_value
    }

    pub fn selected(&self) -> Option<(u8, u8)> {
        match (self.selected_week, self.selected_day) {
            (Some(w), Some(d)) => Some((w, d)),
            _ => None,
        }
    }

    pub fn select(&mut self, week: u8, day: u8) {
        self.selected_week = Some(week.min(52));
        self.selected_day = Some(day.min(6));
    }

    pub fn clear_selection(&mut self) {
        self.selected_week = None;
        self.selected_day = None;
    }

    pub fn move_selection(&mut self, dw: i8, dd: i8) {
        let week = self.selected_week.unwrap_or(0) as i8;
        let day = self.selected_day.unwrap_or(0) as i8;
        let new_week = (week + dw).clamp(0, 52) as u8;
        let new_day = (day + dd).clamp(0, 6) as u8;
        self.select(new_week, new_day);
    }

    pub fn entries_iter(&self) -> impl Iterator<Item = &HeatmapEntry> {
        self.entries.values()
    }
}
