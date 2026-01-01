use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::theme::palette;

use super::state::{ChatMessage, MessageRole};

pub fn render_message(msg: &ChatMessage, area: Rect, buf: &mut Buffer) {
    if area.height < 2 {
        return;
    }

    let (border_color, role_label) = match msg.role {
        MessageRole::User => (palette::CYAN, "You"),
        MessageRole::Assistant => (palette::PURPLE, "Assistant"),
        MessageRole::System => (palette::WARNING, "System"),
    };

    let title = if msg.is_streaming {
        format!("{} \u{F110}", role_label)
    } else {
        role_label.to_string()
    };

    let content = parse_message_content(&msg.content);

    let block = Block::default()
        .title(title)
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false });

    paragraph.render(area, buf);
}

pub fn parse_message_content(content: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    for line in content.lines() {
        if line.starts_with("```") {
            in_code_block = !in_code_block;
            if in_code_block {
                let code_lang = line.trim_start_matches('`');
                let header = format!(
                    "\u{F121} {}",
                    if code_lang.is_empty() { "code" } else { code_lang }
                );
                lines.push(Line::from(Span::styled(
                    header,
                    Style::default().fg(palette::GREEN).add_modifier(Modifier::DIM),
                )));
            }
            continue;
        }

        if in_code_block {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(palette::TEXT_DIM).bg(Color::Rgb(30, 30, 40)),
            )));
        } else {
            lines.push(parse_inline_formatting(line));
        }
    }

    lines
}

fn parse_inline_formatting(line: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '`' => {
                if !current.is_empty() {
                    spans.push(Span::raw(std::mem::take(&mut current)));
                }
                let mut code = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '`' {
                        chars.next();
                        break;
                    }
                    code.push(chars.next().unwrap_or(' '));
                }
                spans.push(Span::styled(code, Style::default().fg(palette::PINK)));
            }
            '*' if chars.peek() == Some(&'*') => {
                chars.next();
                if !current.is_empty() {
                    spans.push(Span::raw(std::mem::take(&mut current)));
                }
                let mut bold = String::new();
                while let Some(nc) = chars.next() {
                    if nc == '*' && chars.peek() == Some(&'*') {
                        chars.next();
                        break;
                    }
                    bold.push(nc);
                }
                spans.push(Span::styled(bold, Style::default().add_modifier(Modifier::BOLD)));
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        spans.push(Span::raw(current));
    }

    Line::from(spans)
}

pub fn message_height(msg: &ChatMessage, width: u16) -> u16 {
    let content_width = width.saturating_sub(4) as usize;
    if content_width == 0 {
        return 1;
    }

    let lines: usize = msg
        .content
        .lines()
        .map(|line| (line.len() / content_width).max(1))
        .sum();

    (lines as u16).max(1) + 1
}
