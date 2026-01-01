use crate::theme::colors::palette;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use super::{
    render::{render_agent_card, render_resource_bar},
    state::AgentsState,
};

pub struct Agents;

impl StatefulWidget for Agents {
    type State = AgentsState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        render_agent_list(chunks[0], buf, state);
        render_agent_detail(chunks[1], buf, state);
    }
}

fn render_agent_list(area: Rect, buf: &mut Buffer, state: &AgentsState) {
    let block = Block::default()
        .title(" \u{F0219} Agents ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette::BORDER));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut y = inner.y;
    for (i, agent) in state.agents.iter().enumerate() {
        if y >= inner.bottom() - 2 {
            break;
        }
        let selected = i == state.selected;
        let lines = render_agent_card(agent, selected);
        for line in lines {
            if y < inner.bottom() {
                buf.set_line(inner.x + 1, y, &line, inner.width.saturating_sub(2));
                y += 1;
            }
        }
        y += 1;
    }
}

fn render_agent_detail(area: Rect, buf: &mut Buffer, state: &AgentsState) {
    let block = Block::default()
        .title(" \u{F064E} Details ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette::BORDER));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(agent) = state.selected_agent() else {
        let empty = Paragraph::new("No agent selected");
        empty.render(inner, buf);
        return;
    };

    let mut lines = vec![];
    lines.push(render_resource_bar("MEM", agent.memory_mb, agent.memory_max_mb, palette::CYAN));
    lines.push(render_resource_bar("TOK", agent.tokens_used, agent.tokens_max, palette::MAGENTA));

    for (i, line) in lines.iter().enumerate() {
        if inner.y + (i as u16) < inner.bottom() {
            buf.set_line(inner.x + 1, inner.y + (i as u16), line, inner.width.saturating_sub(2));
        }
    }
}
