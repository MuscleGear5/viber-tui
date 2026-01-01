use crate::data::Action;
use crate::theme::colors;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

pub struct ActionPreview<'a> {
    action: &'a Action,
    focused: bool,
    border_color: Option<Color>,
    glow: f32,
}

impl<'a> ActionPreview<'a> {
    pub fn new(action: &'a Action) -> Self {
        Self {
            action,
            focused: false,
            border_color: None,
            glow: 0.0,
        }
    }
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
    pub fn glow(mut self, glow: f32) -> Self {
        self.glow = glow;
        self
    }

    fn glow_color(&self, base: Color) -> Color {
        if self.glow <= 0.0 {
            return base;
        }
        match base {
            Color::Rgb(r, g, b) => {
                let boost = (self.glow * 30.0) as u8;
                Color::Rgb(
                    r.saturating_add(boost),
                    g.saturating_add(boost),
                    b.saturating_add(boost),
                )
            }
            _ => base,
        }
    }

    fn render_content(&self) -> Vec<Line<'static>> {
        let cat_color = colors::category_color(self.action.category);
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
            lines.push(Line::from(vec![Span::styled(
                self.action.keywords.join(", "),
                colors::text_dim(),
            )]));
        }
        if !self.action.params.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::from(vec![Span::styled(
                "Parameters:",
                colors::text_muted(),
            )]));
            for p in &self.action.params {
                let req = if p.required { "*" } else { "" };
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {}{}: ", p.name, req),
                        Style::default().fg(colors::ACCENT_CYAN),
                    ),
                    Span::styled(format!("{:?}", p.param_type), colors::text_dim()),
                ]));
            }
        }
        lines
    }
}

impl<'a> Widget for ActionPreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cat_color = colors::category_color(self.action.category);
        let base_border = self.border_color.unwrap_or(colors::BORDER_SUBTLE);
        let border_color = if self.focused {
            self.glow_color(base_border)
        } else {
            base_border
        };
        let title_style = if self.focused {
            Style::default()
                .fg(self.glow_color(cat_color))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(cat_color).add_modifier(Modifier::BOLD)
        };
        let block = Block::default()
            .title(Span::styled(
                format!(" {} Preview ", self.action.name),
                title_style,
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .padding(Padding::uniform(1));
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.height < 4 || inner.width < 20 {
            return;
        }
        Paragraph::new(self.render_content())
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
