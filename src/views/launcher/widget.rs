use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Color,
    widgets::StatefulWidget,
};

use crate::theme::{animation::AnimationState, palette};

use super::preview::render_preview;
use super::render::{render_input_bar, render_results_list};
use super::state::LauncherState;

pub struct Launcher<'a> {
    animation: &'a AnimationState,
}

impl<'a> Launcher<'a> {
    pub fn new(animation: &'a AnimationState) -> Self {
        Self { animation }
    }

    pub fn glow_color(&self, base: Color, intensity: f32) -> Color {
        match base {
            Color::Rgb(r, g, b) => {
                let boost = (intensity * 40.0) as u8;
                Color::Rgb(
                    r.saturating_add(boost),
                    g.saturating_add(boost),
                    b.saturating_add(boost),
                )
            }
            _ => base,
        }
    }

    pub fn pulse_border(&self, base: Color) -> Color {
        self.glow_color(base, self.animation.pulse())
    }

    pub fn animation(&self) -> &AnimationState {
        self.animation
    }
}

impl<'a> StatefulWidget for Launcher<'a> {
    type State = LauncherState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let show_preview = state.show_preview && area.width > 80;

        let main_chunks = if show_preview {
            Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).split(area)
        } else {
            Layout::horizontal([Constraint::Percentage(100)]).split(area)
        };

        let left_chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(main_chunks[0]);

        render_input_bar(&self, left_chunks[0], buf, state);
        render_results_list(&self, left_chunks[1], buf, state);

        if show_preview && main_chunks.len() > 1 {
            render_preview(&self, main_chunks[1], buf, state, palette::PURPLE);
        }
    }
}
