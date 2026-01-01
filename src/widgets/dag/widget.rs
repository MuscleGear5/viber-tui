use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use super::{state::DagState, render::render_dag};

pub struct DagView;

impl StatefulWidget for DagView {
    type State = DagState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        render_dag(area, buf, state);
    }
}
