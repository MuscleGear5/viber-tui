use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartKind {
    #[default]
    Line,
    Bar,
}

#[derive(Debug, Clone)]
pub struct Series {
    pub name: String,
    pub data: Vec<f64>,
    pub color: Color,
}

impl Series {
    pub fn new(name: impl Into<String>, color: Color) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            color,
        }
    }

    pub fn with_data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }
}

#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub kind: ChartKind,
    pub title: Option<String>,
    pub show_legend: bool,
    pub show_x_axis: bool,
    pub show_y_axis: bool,
    pub x_labels: Vec<String>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            kind: ChartKind::Line,
            title: None,
            show_legend: true,
            show_x_axis: true,
            show_y_axis: true,
            x_labels: Vec::new(),
            y_min: None,
            y_max: None,
        }
    }
}

impl ChartConfig {
    pub fn line() -> Self {
        Self::default()
    }

    pub fn bar() -> Self {
        Self {
            kind: ChartKind::Bar,
            ..Default::default()
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_x_labels(mut self, labels: Vec<String>) -> Self {
        self.x_labels = labels;
        self
    }

    pub fn with_y_range(mut self, min: f64, max: f64) -> Self {
        self.y_min = Some(min);
        self.y_max = Some(max);
        self
    }
}
