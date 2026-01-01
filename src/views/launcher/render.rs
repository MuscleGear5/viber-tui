use super::state::{FocusPanel, LauncherState};
use super::widget::Launcher;
use crate::theme::{colors, palette};
use crate::widgets::FuzzyList;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget},
};

pub fn render_input_bar(launcher: &Launcher, area: Rect, buf: &mut Buffer, state: &LauncherState) {
    let is_focused = state.focus == FocusPanel::Input;
    let border_color = if is_focused {
        launcher.pulse_border(palette::CYAN)
    } else {
        palette::BORDER_SUBTLE
    };
    let title_style = if is_focused {
        Style::default()
            .fg(launcher.pulse_border(palette::CYAN))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette::TEXT_MUTED)
    };
    let spinner = if !state.input.is_empty() {
        format!(" {} ", launcher.animation().spinner_braille())
    } else {
        String::new()
    };
    let block = Block::default()
        .title(Span::styled(" Search Actions ", title_style))
        .title_bottom(Line::from(vec![Span::styled(
            spinner,
            Style::default().fg(palette::MAGENTA),
        )]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::horizontal(1));
    let inner = block.inner(area);
    block.render(area, buf);
    let cursor_char = match (is_focused, launcher.animation().cursor_visible()) {
        (true, true) => "\u{258E}",
        (true, false) => " ",
        _ => "",
    };
    let (before, after) = (
        &state.input[..state.cursor_pos],
        &state.input[state.cursor_pos..],
    );
    let prompt_color = if is_focused {
        launcher.pulse_border(palette::MAGENTA)
    } else {
        palette::TEXT_DIM
    };
    let input_line = Line::from(vec![
        Span::styled(
            "> ",
            Style::default()
                .fg(prompt_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(before.to_string(), colors::text_primary()),
        Span::styled(
            cursor_char.to_string(),
            Style::default()
                .fg(palette::CYAN)
                .add_modifier(Modifier::RAPID_BLINK),
        ),
        Span::styled(after.to_string(), colors::text_primary()),
    ]);
    Paragraph::new(input_line).render(inner, buf);
    let count_text = format!(" {} ", state.result_count());
    let count_width = count_text.len() as u16;
    if inner.width > count_width + 10 {
        let count_area = Rect {
            x: inner.x + inner.width - count_width,
            y: inner.y,
            width: count_width,
            height: 1,
        };
        Paragraph::new(Span::styled(
            count_text,
            Style::default().fg(palette::TEXT_DIM).bg(palette::BG_PANEL),
        ))
        .render(count_area, buf);
    }
}

pub fn render_results_list(
    launcher: &Launcher,
    area: Rect,
    buf: &mut Buffer,
    state: &mut LauncherState,
) {
    let is_focused = state.focus == FocusPanel::List;
    let border_color = if is_focused {
        launcher.pulse_border(palette::PURPLE)
    } else {
        palette::BORDER_DIM
    };
    let title = if state.input.is_empty() {
        " All Actions "
    } else {
        " Matches "
    };
    let title_style = if is_focused {
        Style::default()
            .fg(launcher.pulse_border(palette::PURPLE))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette::TEXT_MUTED)
    };
    let block = Block::default()
        .title(Span::styled(title, title_style))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    let fuzzy_list = FuzzyList::new(&state.results)
        .block(block)
        .focused(is_focused)
        .glow(launcher.animation().glow());
    StatefulWidget::render(fuzzy_list, area, buf, &mut state.list_state);
}
