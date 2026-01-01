use crate::theme::colors::palette;
use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestionType {
    Text,
    SingleChoice,
    MultiChoice,
    Number,
    Confirm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    Pending,
}

impl ValidationResult {
    pub fn color(&self) -> Color {
        match self {
            Self::Valid => palette::SUCCESS,
            Self::Invalid(_) => palette::ERROR,
            Self::Pending => palette::TEXT_MUTED,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub id: String,
    pub label: String,
    pub selected: bool,
}

impl Choice {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            selected: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    pub id: String,
    pub prompt: String,
    pub question_type: QuestionType,
    pub choices: Vec<Choice>,
    pub answer: String,
    pub validation: ValidationResult,
    pub required: bool,
}

impl Question {
    pub fn text(id: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            prompt: prompt.into(),
            question_type: QuestionType::Text,
            choices: Vec::new(),
            answer: String::new(),
            validation: ValidationResult::Pending,
            required: true,
        }
    }

    pub fn single_choice(
        id: impl Into<String>,
        prompt: impl Into<String>,
        choices: Vec<Choice>,
    ) -> Self {
        Self {
            id: id.into(),
            prompt: prompt.into(),
            question_type: QuestionType::SingleChoice,
            choices,
            answer: String::new(),
            validation: ValidationResult::Pending,
            required: true,
        }
    }

    pub fn confirm(id: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            prompt: prompt.into(),
            question_type: QuestionType::Confirm,
            choices: Vec::new(),
            answer: String::new(),
            validation: ValidationResult::Pending,
            required: true,
        }
    }

    pub fn validate(&mut self) {
        if self.required && self.answer.is_empty() {
            self.validation = ValidationResult::Invalid("Required field".to_string());
            return;
        }
        if let QuestionType::Number = self.question_type {
            if !self.answer.is_empty() && self.answer.parse::<i64>().is_err() {
                self.validation = ValidationResult::Invalid("Must be a number".to_string());
                return;
            }
        }
        self.validation = ValidationResult::Valid;
    }
}
