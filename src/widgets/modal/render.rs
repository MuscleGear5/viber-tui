use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding};
use super::models::{Modal, ModalKind};

pub fn modal_area(screen: Rect, modal: &Modal) -> Rect {
    let content_lines = modal.message.lines().count() as u16;
    let has_input = modal.input_value.is_some();
    let height = 3 + content_lines + if has_input { 2 } else { 0 } + 2;
    
    let title_len = modal.title.len() as u16 + 4;
    let msg_max = modal.message.lines().map(|l| l.len()).max().unwrap_or(0) as u16;
    let width = title_len.max(msg_max + 4).clamp(modal.min_width, modal.max_width);
    
    let x = screen.x + (screen.width.saturating_sub(width)) / 2;
    let y = screen.y + (screen.height.saturating_sub(height)) / 2;
    
    Rect::new(x, y, width.min(screen.width), height.min(screen.height))
}

pub fn kind_color(kind: ModalKind) -> Color {
    match kind {
        ModalKind::Info => Color::Cyan,
        ModalKind::Warning => Color::Yellow,
        ModalKind::Error => Color::Red,
        ModalKind::Confirm => Color::Blue,
        ModalKind::Input => Color::Magenta,
    }
}

pub fn modal_block(modal: &Modal) -> Block<'static> {
    let color = kind_color(modal.kind);
    let icon = modal.kind.icon();
    let title = format!(" {} {} ", icon, modal.title);
    
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title)
        .title_style(Style::default().fg(color).bold())
        .padding(Padding::horizontal(1))
}

pub fn button_spans(modal: &Modal, selected: usize) -> Line<'static> {
    let mut spans = Vec::new();
    
    for (i, btn) in modal.buttons.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        
        let is_selected = i == selected;
        let base_color = if btn.is_destructive {
            Color::Red
        } else if btn.is_primary {
            Color::Green
        } else {
            Color::White
        };
        
        let style = if is_selected {
            Style::default().fg(Color::Black).bg(base_color).bold()
        } else {
            Style::default().fg(base_color)
        };
        
        let label = format!(" [{}] {} ", btn.key, btn.label);
        spans.push(Span::styled(label, style));
    }
    
    Line::from(spans)
}

pub fn input_line(value: &str, placeholder: Option<&str>, width: u16) -> Line<'static> {
    let display = if value.is_empty() {
        placeholder.unwrap_or("").to_string()
    } else {
        value.to_string()
    };
    
    let style = if value.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };
    
    let content: String = display.chars().take(width as usize - 2).collect();
    let padded = format!(" {:<width$}", content, width = (width as usize).saturating_sub(2));
    
    Line::from(Span::styled(padded, style.bg(Color::Rgb(40, 40, 40))))
}
