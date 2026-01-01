use ratatui::style::Color;

use crate::theme::palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    #[default]
    Info,
    Hint,
}

impl DiagnosticSeverity {
    pub fn color(&self) -> Color {
        match self {
            Self::Error => palette::ERROR,
            Self::Warning => palette::WARNING,
            Self::Info => palette::INFO,
            Self::Hint => palette::TEXT_MUTED,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Error => "\u{F0159}",
            Self::Warning => "\u{F0028}",
            Self::Info => "\u{F064E}",
            Self::Hint => "\u{F0335}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
    pub code: Option<String>,
}

impl Diagnostic {
    pub fn new(file: &str, line: usize, col: usize, sev: DiagnosticSeverity, msg: &str) -> Self {
        Self {
            file: file.to_string(),
            line,
            column: col,
            severity: sev,
            message: msg.to_string(),
            source: None,
            code: None,
        }
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.source = Some(source.to_string());
        self
    }

    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub content: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub preview: String,
}

impl Reference {
    pub fn new(file: &str, line: usize, column: usize, preview: &str) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
            preview: preview.to_string(),
        }
    }

    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
}
