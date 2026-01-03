use ratatui::style::Color;

use super::{ChartConfig, Series};

pub struct ChartState {
    pub series: Vec<Series>,
    pub config: ChartConfig,
    max_points: usize,
}

impl ChartState {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            series: Vec::new(),
            config,
            max_points: 100,
        }
    }

    pub fn with_max_points(mut self, max: usize) -> Self {
        self.max_points = max;
        self
    }

    pub fn add_series(&mut self, name: impl Into<String>, color: Color) -> usize {
        let idx = self.series.len();
        self.series.push(Series::new(name, color));
        idx
    }

    pub fn push(&mut self, series_idx: usize, value: f64) {
        if let Some(series) = self.series.get_mut(series_idx) {
            series.push(value);
            if series.data.len() > self.max_points {
                series.data.remove(0);
            }
        }
    }

    pub fn push_all(&mut self, values: &[f64]) {
        for (idx, &value) in values.iter().enumerate() {
            self.push(idx, value);
        }
    }

    pub fn clear(&mut self) {
        for series in &mut self.series {
            series.data.clear();
        }
    }

    pub fn y_range(&self) -> (f64, f64) {
        if let (Some(min), Some(max)) = (self.config.y_min, self.config.y_max) {
            return (min, max);
        }

        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for series in &self.series {
            for &v in &series.data {
                if v < min {
                    min = v;
                }
                if v > max {
                    max = v;
                }
            }
        }

        if min == f64::MAX {
            (0.0, 1.0)
        } else {
            (
                self.config.y_min.unwrap_or(min),
                self.config.y_max.unwrap_or(max),
            )
        }
    }

    pub fn max_data_len(&self) -> usize {
        self.series.iter().map(|s| s.data.len()).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_series_and_push() {
        let mut state = ChartState::new(ChartConfig::line());
        let idx = state.add_series("CPU", Color::Cyan);
        state.push(idx, 50.0);
        state.push(idx, 75.0);

        assert_eq!(state.series.len(), 1);
        assert_eq!(state.series[0].data, vec![50.0, 75.0]);
    }

    #[test]
    fn test_y_range_auto() {
        let mut state = ChartState::new(ChartConfig::line());
        let idx = state.add_series("Test", Color::White);
        state.push(idx, 10.0);
        state.push(idx, 90.0);

        assert_eq!(state.y_range(), (10.0, 90.0));
    }

    #[test]
    fn test_max_points_limit() {
        let mut state = ChartState::new(ChartConfig::line()).with_max_points(3);
        let idx = state.add_series("Test", Color::White);

        for i in 0..5 {
            state.push(idx, i as f64);
        }

        assert_eq!(state.series[0].data, vec![2.0, 3.0, 4.0]);
    }
}
