use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use crate::theme::colors::palette;
use super::nvim::NvimSyncStatus;
use super::render::{gutter_width, render_code_line, render_line_number};
use super::state::BufferState;

pub struct BufferView;

impl BufferView {
    pub fn new() -> Self { Self }
}

fn render_nvim_status(status: &NvimSyncStatus) -> Span<'static> {
    match status {
        NvimSyncStatus::Disconnected => Span::styled(
            " \u{F0318} Disconnected ",
            Style::default().fg(palette::TEXT_DIM),
        ),
        NvimSyncStatus::Connecting => Span::styled(
            " \u{F0AA8} Connecting... ",
            Style::default().fg(palette::WARNING).add_modifier(Modifier::BOLD),
        ),
        NvimSyncStatus::Synced => Span::styled(
            " \u{F05E0} Synced ",
            Style::default().fg(palette::SUCCESS),
        ),
        NvimSyncStatus::Stale => Span::styled(
            " \u{F0026} Stale ",
            Style::default().fg(palette::ERROR),
        ),
    }
}

impl StatefulWidget for BufferView {
    type State = BufferState;
    
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = state.path.as_deref().unwrap_or("[No File]");
        let modified_indicator = if state.is_modified { " [+]" } else { "" };
        let nvim_status = render_nvim_status(&state.nvim.sync_status);
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette::BORDER_FOCUS))
            .title(Line::from(vec![
                Span::raw(" "),
                Span::styled(title, Style::default().fg(palette::CYAN)),
                Span::raw(modified_indicator),
                Span::raw(" "),
            ]))
            .title_bottom(Line::from(vec![nvim_status]));
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        if inner.width < 10 || inner.height < 1 {
            return;
        }
        
        let chunks = Layout::horizontal([
            Constraint::Length(gutter_width()),
            Constraint::Min(1),
        ])
        .split(inner);
        
        let gutter_area = chunks[0];
        let code_area = chunks[1];
        let visible_height = code_area.height as usize;
        
        let end_line = (state.scroll_offset + visible_height).min(state.lines.len());
        let visible_lines = &state.lines[state.scroll_offset..end_line];
        
        for (i, line) in visible_lines.iter().enumerate() {
            let y = gutter_area.y + i as u16;
            if y >= gutter_area.y + gutter_area.height {
                break;
            }
            
            let is_current = state.scroll_offset + i == state.cursor_line;
            let line_num = render_line_number(line.number, is_current, state.lines.len());
            let gutter_line_area = Rect::new(gutter_area.x, y, gutter_area.width, 1);
            Paragraph::new(line_num).render(gutter_line_area, buf);
            
            let code_line = render_code_line(line, state.language, is_current);
            let code_line_area = Rect::new(code_area.x, y, code_area.width, 1);
            Paragraph::new(code_line).render(code_line_area, buf);
        }
    }
}
