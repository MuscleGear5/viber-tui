use super::state::{FocusPanel, LauncherState};
use super::widget::Launcher;
use crate::theme::{colors, palette};
use crate::widgets::ActionPreview;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render_preview(
    launcher: &Launcher,
    area: Rect,
    buf: &mut Buffer,
    state: &LauncherState,
    fallback: ratatui::style::Color,
) {
    let is_focused = state.focus == FocusPanel::Preview;
    if let Some(action) = state.selected_action() {
        let border = if is_focused {
            launcher.pulse_border(colors::category_color(action.category))
        } else {
            palette::BORDER_DIM
        };
        ActionPreview::new(action)
            .focused(is_focused)
            .border_color(border)
            .glow(launcher.animation().glow())
            .render(area, buf);
    } else {
        let border = if is_focused {
            launcher.pulse_border(fallback)
        } else {
            palette::BORDER_DIM
        };
        let block = Block::default()
            .title(Span::styled(
                " Preview ",
                Style::default().fg(palette::TEXT_DIM),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border));
        let inner = block.inner(area);
        block.render(area, buf);
        Paragraph::new(Span::styled("No action selected", colors::text_muted())).render(inner, buf);
    }
}
