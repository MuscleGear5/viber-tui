use super::models::{Question, ValidationResult};

#[derive(Debug, Clone, Default)]
pub struct QuestionnaireState {
    pub title: String,
    pub questions: Vec<Question>,
    pub current_index: usize,
    pub choice_index: usize,
    pub input_buffer: String,
    pub completed: bool,
}

impl QuestionnaireState {
    pub fn new(title: impl Into<String>, questions: Vec<Question>) -> Self {
        Self {
            title: title.into(),
            questions,
            current_index: 0,
            choice_index: 0,
            input_buffer: String::new(),
            completed: false,
        }
    }

    pub fn current_question(&self) -> Option<&Question> {
        self.questions.get(self.current_index)
    }

    pub fn current_question_mut(&mut self) -> Option<&mut Question> {
        self.questions.get_mut(self.current_index)
    }

    pub fn next(&mut self) {
        let answer = self.input_buffer.clone();
        if let Some(q) = self.questions.get_mut(self.current_index) {
            q.answer = answer;
            q.validate();
        }
        if self.current_index < self.questions.len().saturating_sub(1) {
            self.current_index += 1;
            self.choice_index = 0;
            self.input_buffer = self.questions[self.current_index].answer.clone();
        }
    }

    pub fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.choice_index = 0;
            self.input_buffer = self.questions[self.current_index].answer.clone();
        }
    }

    pub fn choice_up(&mut self) {
        if self.choice_index > 0 {
            self.choice_index -= 1;
        }
    }

    pub fn choice_down(&mut self) {
        if let Some(q) = self.current_question() {
            if self.choice_index < q.choices.len().saturating_sub(1) {
                self.choice_index += 1;
            }
        }
    }

    pub fn toggle_choice(&mut self) {
        let idx = self.choice_index;
        if let Some(q) = self.questions.get_mut(self.current_index) {
            let is_single = matches!(q.question_type, super::models::QuestionType::SingleChoice);
            let is_multi = matches!(q.question_type, super::models::QuestionType::MultiChoice);
            
            if is_single {
                for (i, c) in q.choices.iter_mut().enumerate() {
                    c.selected = i == idx;
                }
                q.answer = q.choices.get(idx).map(|c| c.id.clone()).unwrap_or_default();
            } else if is_multi {
                if let Some(choice) = q.choices.get_mut(idx) {
                    choice.selected = !choice.selected;
                }
                q.answer = q.choices.iter()
                    .filter(|c| c.selected)
                    .map(|c| c.id.clone())
                    .collect::<Vec<_>>()
                    .join(",");
            }
        }
    }

    pub fn submit(&mut self) -> bool {
        for q in &mut self.questions {
            q.validate();
            if matches!(q.validation, ValidationResult::Invalid(_)) {
                return false;
            }
        }
        self.completed = true;
        true
    }
}
