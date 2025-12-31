use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::data::{Action, ActionCategory};
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
            Constraint::Length(1), // Icon + Name + Category badge
            Constraint::Length(1), // Description
            Constraint::Min(0),    // Keywords / Details
        ])
        .split(inner);

        // Line 1: Icon + Name + Category
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

        // Line 2: Description
        let desc_line = Line::from(vec![Span::styled(
            self.action.description.clone(),
            colors::text_secondary(),
        )]);
        Paragraph::new(desc_line).render(chunks[1], buf);

        // Line 3+: Keywords or invocation details
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

pub struct ActionPreview<'a> {
    action: &'a Action,
}

impl<'a> ActionPreview<'a> {
    pub fn new(action: &'a Action) -> Self {
        Self { action }
    }
}

impl<'a> Widget for ActionPreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cat_color = colors::category_color(self.action.category);

        let block = Block::default()
            .title(Span::styled(
                format!(" {} Preview ", self.action.name),
                Style::default().fg(cat_color).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors::BORDER_SUBTLE))
            .padding(Padding::uniform(1));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 4 || inner.width < 20 {
            return;
        }

        let mut lines = vec![
            Line::from(vec![
                Span::styled("Category: ", colors::text_muted()),
                Span::styled(
                    self.action.category.as_str().to_string(),
                    Style::default().fg(cat_color),
                ),
            ]),
            Line::from(vec![
                Span::styled("ID: ", colors::text_muted()),
                Span::styled(self.action.id.clone(), colors::text_secondary()),
            ]),
            Line::raw(""),
            Line::from(vec![Span::styled(
                self.action.description.clone(),
                colors::text_primary(),
            )]),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Invocation: ", colors::text_muted()),
                Span::styled(
                    self.action.invocation.clone(),
                    Style::default().fg(colors::ACCENT_GREEN),
                ),
            ]),
        ];

        if !self.action.keywords.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::from(vec![Span::styled(
                "Keywords:",
                colors::text_muted(),
            )]));
            let kw_text = self.action.keywords.join(", ");
            lines.push(Line::from(vec![Span::styled(kw_text, colors::text_dim())]));
        }

        if !self.action.params.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::from(vec![Span::styled(
                "Parameters:",
                colors::text_muted(),
            )]));
            for param in &self.action.params {
                let required = if param.required { "*" } else { "" };
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {}{}: ", param.name, required),
                        Style::default().fg(colors::ACCENT_CYAN),
                    ),
                    Span::styled(format!("{:?}", param.param_type), colors::text_dim()),
                ]));
            }
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
