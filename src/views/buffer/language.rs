use ratatui::style::Color;
use crate::theme::colors::palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Json,
    Yaml,
    Markdown,
    Plain,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "rs" => Self::Rust,
            "py" => Self::Python,
            "ts" | "tsx" => Self::TypeScript,
            "js" | "jsx" => Self::JavaScript,
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "md" => Self::Markdown,
            _ => Self::Plain,
        }
    }
    
    pub fn keyword_color(&self) -> Color {
        match self {
            Self::Rust => palette::MAGENTA,
            Self::Python => palette::CYAN,
            Self::TypeScript | Self::JavaScript => palette::CYAN,
            _ => palette::MAGENTA,
        }
    }
    
    pub fn string_color(&self) -> Color {
        palette::GREEN
    }
    
    pub fn comment_color(&self) -> Color {
        palette::TEXT_DIM
    }
    
    pub fn number_color(&self) -> Color {
        palette::WARNING
    }
}
