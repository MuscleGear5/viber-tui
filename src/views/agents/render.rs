use crate::theme::colors::palette;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use super::state::{Agent, AgentStatus};

pub fn render_agent_card(agent: &Agent, selected: bool) -> Vec<Line<'static>> {
    let border_color = if selected { palette::CYAN } else { palette::BORDER };
    let status_icon = agent.status.icon();
    let status_color = agent.status.color();
    
    let name_line = Line::from(vec![
        Span::styled(format!("{} ", status_icon), Style::default().fg(status_color)),
        Span::styled(agent.name.clone(), Style::default().fg(palette::CYAN)),
    ]);
    
    let task_text = agent.current_task.clone().unwrap_or_else(|| "idle".to_string());
    let task_line = Line::from(vec![
        Span::styled("  \u{F0493} ", Style::default().fg(palette::TEXT_MUTED)),
        Span::styled(task_text, Style::default().fg(palette::GREEN)),
    ]);
    
    vec![name_line, task_line]
}

pub fn render_resource_bar(label: &str, current: u32, max: u32, color: Color) -> Line<'static> {
    let percent = if max == 0 { 0.0 } else { (current as f64 / max as f64) * 100.0 };
    let bar_width = 20;
    let filled = ((percent / 100.0) * bar_width as f64) as usize;
    let empty = bar_width - filled;
    
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    let value_text = format!(" {}/{}", format_number(current), format_number(max));
    
    Line::from(vec![
        Span::styled(format!("{}: ", label), Style::default().fg(palette::TEXT_MUTED)),
        Span::styled(bar, Style::default().fg(color)),
        Span::styled(value_text, Style::default().fg(palette::GREEN)),
    ])
}

fn format_number(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
