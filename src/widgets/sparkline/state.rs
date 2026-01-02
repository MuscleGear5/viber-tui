use super::models::{DataPoint, SparklineConfig};

#[derive(Debug, Clone)]
pub struct SparklineState {
    pub data: Vec<DataPoint>,
    pub config: SparklineConfig,
    pub max_points: usize,
    cached_min: Option<f64>,
    cached_max: Option<f64>,
}

impl Default for SparklineState {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            config: SparklineConfig::default(),
            max_points: 50,
            cached_min: None,
            cached_max: None,
        }
    }
}

impl SparklineState {
    pub fn new(config: SparklineConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn with_max_points(mut self, max: usize) -> Self {
        self.max_points = max;
        self
    }

    pub fn push(&mut self, point: impl Into<DataPoint>) {
        self.data.push(point.into());
        if self.data.len() > self.max_points {
            self.data.remove(0);
        }
        self.invalidate_cache();
    }

    pub fn push_many(&mut self, points: impl IntoIterator<Item = impl Into<DataPoint>>) {
        for point in points {
            self.data.push(point.into());
        }
        while self.data.len() > self.max_points {
            self.data.remove(0);
        }
        self.invalidate_cache();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.invalidate_cache();
    }

    fn invalidate_cache(&mut self) {
        self.cached_min = None;
        self.cached_max = None;
    }

    pub fn min(&mut self) -> Option<f64> {
        if self.cached_min.is_none() {
            self.cached_min = self.data.iter().map(|p| p.value).reduce(f64::min);
        }
        self.cached_min
    }

    pub fn max(&mut self) -> Option<f64> {
        if self.cached_max.is_none() {
            self.cached_max = self.data.iter().map(|p| p.value).reduce(f64::max);
        }
        self.cached_max
    }

    pub fn current(&self) -> Option<f64> {
        self.data.last().map(|p| p.value)
    }

    pub fn range(&mut self) -> Option<(f64, f64)> {
        match (self.min(), self.max()) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparkline_empty() {
        let mut state = SparklineState::default();
        assert!(state.current().is_none());
        assert!(state.min().is_none());
        assert!(state.max().is_none());
    }

    #[test]
    fn sparkline_push_and_stats() {
        let mut state = SparklineState::default();
        state.push(DataPoint::new(10.0));
        state.push(DataPoint::new(20.0));
        state.push(DataPoint::new(5.0));
        assert_eq!(state.current(), Some(5.0));
        assert_eq!(state.min(), Some(5.0));
        assert_eq!(state.max(), Some(20.0));
        assert_eq!(state.range(), Some((5.0, 20.0)));
    }

    #[test]
    fn sparkline_max_points() {
        let mut state = SparklineState::default().with_max_points(3);
        state.push_many([1.0, 2.0, 3.0, 4.0, 5.0].map(DataPoint::new));
        assert_eq!(state.data.len(), 3);
        assert_eq!(state.current(), Some(5.0));
    }

    #[test]
    fn sparkline_clear() {
        let mut state = SparklineState::default();
        state.push(DataPoint::new(100.0));
        state.clear();
        assert!(state.data.is_empty());
        assert!(state.current().is_none());
    }
}
