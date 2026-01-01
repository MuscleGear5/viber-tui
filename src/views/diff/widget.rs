use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::StatefulWidget,
};

use super::state::DiffState;
use super::render::{render_diff_line, render_hunk_header};

pub struct DiffView {
    pub focused: bool,
}

impl DiffView {
    pub fn new() -> Self {
        Self { focused: false }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl Default for DiffView {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for DiffView {
    type State = DiffState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let mut y = area.y;
        let max_y = area.y + area.height;

        for (hunk_idx, hunk) in state.hunks.iter().enumerate() {
            if y >= max_y {
                break;
            }

            let is_selected = hunk_idx == state.selected_hunk;
            let header_line = render_hunk_header(hunk, area.width, is_selected);
            buf.set_line(area.x, y, &header_line, area.width);
            y += 1;

            for line in &hunk.lines {
                if y >= max_y {
                    break;
                }
                let diff_line = render_diff_line(line, area.width, is_selected);
                buf.set_line(area.x, y, &diff_line, area.width);
                y += 1;
            }

            if y < max_y {
                y += 1;
            }
        }
    }
}
