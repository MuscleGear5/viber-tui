use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::theme::colors::palette;
use super::render::{render_action_bar, render_section_content, render_section_header, render_title};
use super::state::SpecState;

pub struct SpecView;

impl StatefulWidget for SpecView {
    type State = SpecState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette::BORDER))
            .title(" Spec ")
            .title_style(Style::default().fg(palette::CYAN).add_modifier(Modifier::BOLD));
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 4 {
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(inner);

        render_title(state, chunks[0], buf);

        let sections_area = chunks[1];
        let mut y = sections_area.y;

        for (idx, section) in state.sections.iter().enumerate() {
            if y >= sections_area.y + sections_area.height {
                break;
            }

            let selected = idx == state.selected_section;
            let header_area = Rect::new(sections_area.x, y, sections_area.width, 1);
            render_section_header(section, selected, header_area, buf);
            y += 1;

            if !section.collapsed {
                let content_lines = section.content.lines().count().min(5) as u16;
                let remaining = (sections_area.y + sections_area.height).saturating_sub(y);
                let content_height = content_lines.min(remaining);

                if content_height > 0 {
                    let content_area = Rect::new(sections_area.x, y, sections_area.width, content_height);
                    render_section_content(section, content_area, buf, 0);
                    y += content_height;
                }
            }

            y += 1;
        }

        render_action_bar(chunks[2], buf);
    }
}
