use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use super::{render::render_sparkline, state::SparklineState};

pub struct Sparkline;

impl StatefulWidget for Sparkline {
    type State = SparklineState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        render_sparkline(state, area, buf);
    }
}
