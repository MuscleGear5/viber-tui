use ratatui::style::Color;

use crate::theme::colors::palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparklineStyle {
    Line,
    Bar,
    Dot,
}

impl Default for SparklineStyle {
    fn default() -> Self {
        Self::Line
    }
}

#[derive(Debug, Clone)]
pub struct SparklineConfig {
    pub style: SparklineStyle,
    pub color: Color,
    pub show_min_max: bool,
    pub show_current: bool,
    pub baseline: Option<f64>,
}

impl Default for SparklineConfig {
    fn default() -> Self {
        Self {
            style: SparklineStyle::Line,
            color: palette::CYAN,
            show_min_max: false,
            show_current: true,
            baseline: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub value: f64,
    pub label: Option<String>,
}

impl DataPoint {
    pub fn new(value: f64) -> Self {
        Self { value, label: None }
    }

    pub fn with_label(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: Some(label.into()),
        }
    }
}

impl From<f64> for DataPoint {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<i64> for DataPoint {
    fn from(value: i64) -> Self {
        Self::new(value as f64)
    }
}
