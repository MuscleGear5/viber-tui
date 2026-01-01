use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::data::Action;
use crate::theme::{colors, palette};

pub fn render_action_line(
    action: &Action,
    match_indices: &[u32],
    is_selected: bool,
    focused: bool,
    glow: f32,
) -> Line<'static> {
    let icon = action.display_icon();
    let cat_color = colors::category_color(action.category);

    let icon_style = if is_selected && focused {
        let boost = (glow * 30.0) as u8;
        match cat_color {
            Color::Rgb(r, g, b) => Style::default().fg(Color::Rgb(
                r.saturating_add(boost),
                g.saturating_add(boost),
                b.saturating_add(boost),
            )),
            _ => Style::default().fg(cat_color),
        }
    } else {
        Style::default().fg(cat_color)
    };

    let icon_span = Span::styled(format!("{} ", icon), icon_style);
    let name = &action.name;
    let mut name_spans = Vec::new();
    let mut last_idx = 0;
    let name_len = name.len();
    let highlight_color = if is_selected && focused {
        palette::CYAN
    } else {
        colors::ACCENT_CYAN
    };

    for &idx in match_indices {
        let idx = idx as usize;
        if idx >= name_len {
            break;
        }
        if idx > last_idx {
            name_spans.push(Span::styled(
                name[last_idx..idx].to_string(),
                colors::text_primary(),
            ));
        }
        name_spans.push(Span::styled(
            name[idx..idx + 1].to_string(),
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
        last_idx = idx + 1;
    }
    if last_idx < name_len {
        name_spans.push(Span::styled(
            name[last_idx..].to_string(),
            colors::text_primary(),
        ));
    }

    let desc_style = if is_selected {
        colors::text_secondary()
    } else {
        colors::text_muted()
    };
    let desc_span = Span::styled(format!("  {}", action.description), desc_style);

    let mut spans = vec![icon_span];
    spans.extend(name_spans);
    spans.push(desc_span);

    Line::from(spans)
}
