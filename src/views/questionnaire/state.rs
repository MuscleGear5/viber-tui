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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::models::Choice;

    fn sample_questions() -> Vec<Question> {
        vec![
            Question::text("name", "What is your name?"),
            Question::single_choice(
                "color",
                "Pick a color",
                vec![
                    Choice::new("r", "Red"),
                    Choice::new("g", "Green"),
                    Choice::new("b", "Blue"),
                ],
            ),
        ]
    }

    #[test]
    fn test_new_and_current_question() {
        let state = QuestionnaireState::new("Test", sample_questions());
        assert_eq!(state.title, "Test");
        assert_eq!(state.current_index, 0);
        assert!(state.current_question().is_some());
        assert_eq!(state.current_question().unwrap().id, "name");
    }

    #[test]
    fn test_navigation_next_prev() {
        let mut state = QuestionnaireState::new("Nav", sample_questions());
        state.input_buffer = "Alice".to_string();
        state.next();
        assert_eq!(state.current_index, 1);
        assert_eq!(state.questions[0].answer, "Alice");
        state.prev();
        assert_eq!(state.current_index, 0);
        state.prev();
        assert_eq!(state.current_index, 0);
    }

    #[test]
    fn test_choice_navigation_and_toggle() {
        let mut state = QuestionnaireState::new("Choice", sample_questions());
        state.next();
        assert_eq!(state.choice_index, 0);
        state.choice_down();
        assert_eq!(state.choice_index, 1);
        state.choice_down();
        assert_eq!(state.choice_index, 2);
        state.choice_down();
        assert_eq!(state.choice_index, 2);
        state.choice_up();
        assert_eq!(state.choice_index, 1);
        state.toggle_choice();
        assert_eq!(state.questions[1].answer, "g");
        assert!(state.questions[1].choices[1].selected);
    }

    #[test]
    fn test_submit_validation() {
        let mut state = QuestionnaireState::new("Submit", sample_questions());
        assert!(!state.submit());
        assert!(!state.completed);
        state.input_buffer = "Bob".to_string();
        state.next();
        state.toggle_choice();
        assert!(state.submit());
        assert!(state.completed);
    }
}
