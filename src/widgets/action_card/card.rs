use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::data::Action;
use crate::theme::colors;

pub struct ActionCard<'a> {
    action: &'a Action,
    selected: bool,
    show_details: bool,
}

impl<'a> ActionCard<'a> {
    pub fn new(action: &'a Action) -> Self {
        Self {
            action,
            selected: false,
            show_details: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn show_details(mut self, show: bool) -> Self {
        self.show_details = show;
        self
    }

    fn category_badge(&self) -> Span<'static> {
        let cat = self.action.category;
        let color = colors::category_color(cat);
        Span::styled(
            format!(" {} ", cat.as_str()),
            Style::default()
                .fg(colors::BG_VOID)
                .bg(color)
                .add_modifier(Modifier::BOLD),
        )
    }

    fn keyword_spans(&self) -> Vec<Span<'static>> {
        self.action
            .keywords
            .iter()
            .take(5)
            .flat_map(|kw| {
                vec![
                    Span::styled(
                        format!(" {} ", kw),
                        Style::default()
                            .fg(colors::TEXT_DIM)
                            .bg(colors::BG_ELEVATED),
                    ),
                    Span::raw(" "),
                ]
            })
            .collect()
    }
}

impl<'a> Widget for ActionCard<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cat_color = colors::category_color(self.action.category);

        let border_style = if self.selected {
            Style::default().fg(cat_color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(colors::BORDER_DIM)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 || inner.width < 10 {
            return;
        }

        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

        let icon = self.action.display_icon();
        let header_line = Line::from(vec![
            Span::styled(format!("{} ", icon), Style::default().fg(cat_color)),
            Span::styled(
                self.action.name.clone(),
                colors::text_primary().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            self.category_badge(),
        ]);
        Paragraph::new(header_line).render(chunks[0], buf);

        let desc_line = Line::from(vec![Span::styled(
            self.action.description.clone(),
            colors::text_secondary(),
        )]);
        Paragraph::new(desc_line).render(chunks[1], buf);

        if chunks[2].height > 0 {
            if self.show_details {
                let detail_lines = vec![Line::from(vec![
                    Span::styled("Invoke: ", colors::text_muted()),
                    Span::styled(
                        self.action.invocation.clone(),
                        Style::default().fg(colors::ACCENT_GREEN),
                    ),
                ])];
                Paragraph::new(detail_lines)
                    .wrap(Wrap { trim: true })
                    .render(chunks[2], buf);
            } else {
                let kw_line = Line::from(self.keyword_spans());
                Paragraph::new(kw_line).render(chunks[2], buf);
            }
        }
    }
}
