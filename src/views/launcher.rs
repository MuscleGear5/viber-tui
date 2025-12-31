use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget},
};
use std::sync::Arc;

use crate::data::{Action, ActionRegistry};
use crate::theme::{animation::AnimationState, colors, palette};
use crate::widgets::{ActionPreview, FuzzyList, FuzzyListState, FuzzyMatcher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPanel {
    Input,
    List,
    Preview,
}

pub struct LauncherState {
    pub input: String,
    pub cursor_pos: usize,
    pub fuzzy_matcher: FuzzyMatcher,
    pub list_state: FuzzyListState,
    pub results: Vec<(Arc<Action>, Vec<u32>)>,
    pub show_preview: bool,
    pub focus: FocusPanel,
}

impl LauncherState {
    pub fn new(registry: &ActionRegistry) -> Self {
        let mut fuzzy_matcher = FuzzyMatcher::new();
        fuzzy_matcher.set_actions(registry.iter().cloned().collect());

        let mut state = Self {
            input: String::new(),
            cursor_pos: 0,
            fuzzy_matcher,
            list_state: FuzzyListState::new(),
            results: Vec::new(),
            show_preview: true,
            focus: FocusPanel::Input,
        };
        state.refresh_results();
        state
    }

    pub fn cycle_focus(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Input => FocusPanel::List,
            FocusPanel::List => {
                if self.show_preview {
                    FocusPanel::Preview
                } else {
                    FocusPanel::Input
                }
            }
            FocusPanel::Preview => FocusPanel::Input,
        };
    }

    pub fn cycle_focus_reverse(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Input => {
                if self.show_preview {
                    FocusPanel::Preview
                } else {
                    FocusPanel::List
                }
            }
            FocusPanel::List => FocusPanel::Input,
            FocusPanel::Preview => FocusPanel::List,
        };
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
        self.on_input_changed();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            let prev_char_boundary = self.input[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(prev_char_boundary);
            self.cursor_pos = prev_char_boundary;
            self.on_input_changed();
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.input.remove(self.cursor_pos);
            self.on_input_changed();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos = self.input[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos = self.input[self.cursor_pos..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor_pos + i)
                .unwrap_or(self.input.len());
        }
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = self.input.len();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_pos = 0;
        self.on_input_changed();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next(self.results.len());
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous(self.results.len());
    }

    pub fn select_first(&mut self) {
        self.list_state.select_first();
    }

    pub fn select_last(&mut self) {
        self.list_state.select_last(self.results.len());
    }

    pub fn page_down(&mut self) {
        self.list_state.page_down(self.results.len());
    }

    pub fn page_up(&mut self) {
        self.list_state.page_up(self.results.len());
    }

    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
        if !self.show_preview && self.focus == FocusPanel::Preview {
            self.focus = FocusPanel::List;
        }
    }

    pub fn selected_action(&self) -> Option<&Action> {
        self.list_state
            .selected()
            .and_then(|i| self.results.get(i))
            .map(|(action, _)| action.as_ref())
    }

    pub fn tick(&mut self) {
        if self.fuzzy_matcher.tick() {
            self.refresh_results();
        }
    }

    fn on_input_changed(&mut self) {
        self.fuzzy_matcher.update_pattern(&self.input);
        self.list_state.select_first();
    }

    fn refresh_results(&mut self) {
        self.results = self.fuzzy_matcher.results();
        if self
            .list_state
            .selected()
            .map(|i| i >= self.results.len())
            .unwrap_or(false)
        {
            self.list_state.select_first();
        }
    }

    pub fn result_count(&self) -> usize {
        self.results.len()
    }
}

pub struct Launcher<'a> {
    animation: &'a AnimationState,
}

impl<'a> Launcher<'a> {
    pub fn new(animation: &'a AnimationState) -> Self {
        Self { animation }
    }

    fn glow_color(&self, base: Color, intensity: f32) -> Color {
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

    fn pulse_border(&self, base: Color) -> Color {
        self.glow_color(base, self.animation.pulse())
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

        self.render_input_bar(left_chunks[0], buf, state);
        self.render_results_list(left_chunks[1], buf, state);

        if show_preview && main_chunks.len() > 1 {
            self.render_preview(main_chunks[1], buf, state);
        }
    }
}

impl<'a> Launcher<'a> {
    fn render_input_bar(&self, area: Rect, buf: &mut Buffer, state: &LauncherState) {
        let is_focused = state.focus == FocusPanel::Input;
        let border_color = if is_focused {
            self.pulse_border(palette::CYAN)
        } else {
            palette::BORDER_SUBTLE
        };

        let title_style = if is_focused {
            Style::default()
                .fg(self.pulse_border(palette::CYAN))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette::TEXT_MUTED)
        };

        let spinner = if !state.input.is_empty() {
            format!(" {} ", self.animation.spinner_braille())
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

        let cursor_char = if is_focused && self.animation.cursor_visible() {
            "▎"
        } else if is_focused {
            " "
        } else {
            ""
        };

        let before_cursor = &state.input[..state.cursor_pos];
        let after_cursor = &state.input[state.cursor_pos..];

        let prompt_color = if is_focused {
            self.pulse_border(palette::MAGENTA)
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
            Span::styled(before_cursor.to_string(), colors::text_primary()),
            Span::styled(
                cursor_char.to_string(),
                Style::default()
                    .fg(palette::CYAN)
                    .add_modifier(Modifier::RAPID_BLINK),
            ),
            Span::styled(after_cursor.to_string(), colors::text_primary()),
        ]);

        let count_text = format!(" {} ", state.result_count());
        let count_width = count_text.len() as u16;

        Paragraph::new(input_line).render(inner, buf);

        if inner.width > count_width + 10 {
            let count_area = Rect {
                x: inner.x + inner.width - count_width,
                y: inner.y,
                width: count_width,
                height: 1,
            };
            let count_style = Style::default().fg(palette::TEXT_DIM).bg(palette::BG_PANEL);
            Paragraph::new(Span::styled(count_text, count_style)).render(count_area, buf);
        }
    }

    fn render_results_list(&self, area: Rect, buf: &mut Buffer, state: &mut LauncherState) {
        let is_focused = state.focus == FocusPanel::List;
        let border_color = if is_focused {
            self.pulse_border(palette::PURPLE)
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
                .fg(self.pulse_border(palette::PURPLE))
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
            .glow(self.animation.glow());
        StatefulWidget::render(fuzzy_list, area, buf, &mut state.list_state);
    }

    fn render_preview(&self, area: Rect, buf: &mut Buffer, state: &LauncherState) {
        let is_focused = state.focus == FocusPanel::Preview;

        if let Some(action) = state.selected_action() {
            let border_color = if is_focused {
                self.pulse_border(colors::category_color(action.category))
            } else {
                palette::BORDER_DIM
            };
            ActionPreview::new(action)
                .focused(is_focused)
                .border_color(border_color)
                .glow(self.animation.glow())
                .render(area, buf);
        } else {
            let border_color = if is_focused {
                self.pulse_border(palette::PURPLE)
            } else {
                palette::BORDER_DIM
            };

            let block = Block::default()
                .title(Span::styled(
                    " Preview ",
                    Style::default().fg(palette::TEXT_DIM),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color));

            let inner = block.inner(area);
            block.render(area, buf);

            let msg = Paragraph::new(Span::styled("No action selected", colors::text_muted()));
            msg.render(inner, buf);
        }
    }
}
