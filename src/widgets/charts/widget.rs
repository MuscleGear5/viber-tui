use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use super::{render::render_chart, ChartState};

pub struct Chart;

impl StatefulWidget for Chart {
    type State = ChartState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        render_chart(area, buf, state);
    }
}
