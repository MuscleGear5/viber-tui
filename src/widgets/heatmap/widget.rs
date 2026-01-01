use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::StatefulWidget,
};

use super::{render::render_heatmap, state::HeatmapState};

pub struct Heatmap;

impl StatefulWidget for Heatmap {
    type State = HeatmapState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        render_heatmap(area, buf, state);
    }
}
