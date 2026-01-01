use super::models::{Question, ValidationResult};

#[derive(Debug, Clone, Default)]
pub struct QuestionnaireState {
    pub title: String,
    pub questions: Vec<Question>,
    pub current_index: usize,
    pub input_buffer: String,
    pub completed: bool,
}

impl QuestionnaireState {
    pub fn new(title: impl Into<String>, questions: Vec<Question>) -> Self {
        Self {
            title: title.into(),
            questions,
            current_index: 0,
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
            self.input_buffer = self.questions[self.current_index].answer.clone();
        }
    }

    pub fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.input_buffer = self.questions[self.current_index].answer.clone();
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
