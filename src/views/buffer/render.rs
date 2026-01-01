use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use crate::theme::colors::palette;
use super::language::Language;
use super::state::BufferLine;

const GUTTER_WIDTH: u16 = 6;

pub fn render_line_number(line_num: usize, is_current: bool, max_lines: usize) -> Line<'static> {
    let width = max_lines.to_string().len();
    let num_str = format!("{:>width$} ", line_num, width = width);
    let style = if is_current {
        Style::default().fg(palette::CYAN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette::TEXT_DIM)
    };
    Line::from(Span::styled(num_str, style))
}

pub fn render_code_line(line: &BufferLine, language: Language, is_current: bool) -> Line<'static> {
    let content = &line.content;
    let spans = tokenize_line(content, language);
    
    let mut styled_spans: Vec<Span<'static>> = spans
        .into_iter()
        .map(|(text, color)| Span::styled(text, Style::default().fg(color)))
        .collect();
    
    if is_current {
        for span in &mut styled_spans {
            span.style = span.style.bg(palette::BG_SURFACE);
        }
    }
    
    if line.has_diagnostic {
        let indicator = Span::styled(
            " \u{F0026}",  // nf-md-alert_circle
            Style::default().fg(palette::WARNING),
        );
        styled_spans.push(indicator);
    }
    
    Line::from(styled_spans)
}

fn tokenize_line(content: &str, language: Language) -> Vec<(String, Color)> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let ch = chars[i];
        
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            let comment: String = chars[i..].iter().collect();
            tokens.push((comment, language.comment_color()));
            break;
        }
        
        if ch == '#' && matches!(language, Language::Python | Language::Yaml) {
            let comment: String = chars[i..].iter().collect();
            tokens.push((comment, language.comment_color()));
            break;
        }
        
        if ch == '"' || ch == '\'' {
            let quote = ch;
            let start = i;
            i += 1;
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                }
                i += 1;
            }
            i += 1;
            let string: String = chars[start..i.min(chars.len())].iter().collect();
            tokens.push((string, language.string_color()));
            continue;
        }
        
        if ch.is_ascii_digit() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                i += 1;
            }
            let num: String = chars[start..i].iter().collect();
            tokens.push((num, language.number_color()));
            continue;
        }
        
        if ch.is_alphabetic() || ch == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let color = if is_keyword(&word, language) {
                language.keyword_color()
            } else {
                palette::TEXT_PRIMARY
            };
            tokens.push((word, color));
            continue;
        }
        
        tokens.push((ch.to_string(), palette::TEXT_PRIMARY));
        i += 1;
    }
    
    if tokens.is_empty() {
        tokens.push((String::new(), palette::TEXT_PRIMARY));
    }
    
    tokens
}

fn is_keyword(word: &str, language: Language) -> bool {
    match language {
        Language::Rust => matches!(
            word,
            "fn" | "let" | "mut" | "const" | "pub" | "use" | "mod" | "struct" | "enum" 
            | "impl" | "trait" | "where" | "for" | "if" | "else" | "match" | "loop" 
            | "while" | "return" | "self" | "Self" | "async" | "await" | "move"
        ),
        Language::Python => matches!(
            word,
            "def" | "class" | "if" | "elif" | "else" | "for" | "while" | "return" 
            | "import" | "from" | "as" | "try" | "except" | "finally" | "with" 
            | "async" | "await" | "yield" | "lambda" | "True" | "False" | "None"
        ),
        Language::TypeScript | Language::JavaScript => matches!(
            word,
            "function" | "const" | "let" | "var" | "if" | "else" | "for" | "while" 
            | "return" | "import" | "export" | "from" | "class" | "extends" 
            | "async" | "await" | "try" | "catch" | "throw" | "new" | "this"
        ),
        _ => false,
    }
}

pub fn gutter_width() -> u16 {
    GUTTER_WIDTH
}
