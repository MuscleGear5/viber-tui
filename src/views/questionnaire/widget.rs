use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Padding, StatefulWidget, Widget},
};

use crate::theme::colors::palette;
use super::render::{
    render_choice_list, render_confirm_buttons, render_progress_bar,
    render_question_prompt, render_text_input, render_validation_message,
};
use super::models::QuestionType;
use super::state::QuestionnaireState;

pub struct QuestionnaireView;

impl StatefulWidget for QuestionnaireView {
    type State = QuestionnaireState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .title(format!(" {} ", state.title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette::BORDER))
            .padding(Padding::uniform(1));
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 6 {
            return;
        }

        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

        render_progress_bar(state.current_index, state.questions.len(), chunks[0], buf);

        if let Some(question) = state.current_question() {
            render_question_prompt(question, chunks[2], buf, true);
            match question.question_type {
                QuestionType::Text | QuestionType::Number => {
                    render_text_input(&state.input_buffer, chunks[3], buf, true);
                }
                QuestionType::SingleChoice | QuestionType::MultiChoice => {
                    let choices: Vec<_> = question
                        .choices
                        .iter()
                        .map(|c| (c.id.clone(), c.label.clone(), c.selected))
                        .collect();
                    render_choice_list(&choices, chunks[3], buf, Some(state.choice_index));
                }
                QuestionType::Confirm => {
                    let confirmed = question.answer == "yes";
                    render_confirm_buttons(confirmed, chunks[3], buf, true);
                }
            }
            render_validation_message(&question.validation, chunks[4], buf);
        }

        let nav_hint = "[Tab] Next  [Shift+Tab] Prev  [Enter] Submit";
        buf.set_string(
            chunks[5].x,
            chunks[5].y,
            nav_hint,
            Style::default().fg(palette::TEXT_MUTED),
        );
    }
}
