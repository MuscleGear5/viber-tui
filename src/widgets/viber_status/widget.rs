use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::theme::AnimationState;
use super::render;
use super::state::ViberState;

pub struct ViberStatusPanel<'a> {
    state: &'a ViberState,
    animation: &'a AnimationState,
}

impl<'a> ViberStatusPanel<'a> {
    pub fn new(state: &'a ViberState, animation: &'a AnimationState) -> Self {
        Self { state, animation }
    }
}

impl Widget for ViberStatusPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let eye_color = render::eye_color(self.state, self.animation);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(eye_color))
            .title(Span::styled(
                format!(" {} VIBER ", render::viber_eye(self.animation)),
                Style::default().fg(eye_color).bold(),
            ));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 || inner.width < 10 {
            return;
        }

        let mut lines = vec![
            render::phase_line(self.state),
            render::agents_line(self.state),
            render::vibe_line(self.state),
        ];

        if let Some(intervention) = render::intervention_line(self.state) {
            lines.push(intervention);
        }

        let content = Paragraph::new(lines);
        content.render(inner, buf);
    }
}
