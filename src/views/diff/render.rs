use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};

use crate::theme::palette;
use super::state::{DiffLine, DiffLineKind, DiffHunk};

const GUTTER_WIDTH: u16 = 5;

pub fn render_diff_line(line: &DiffLine, width: u16, selected: bool) -> Line<'static> {
    let half_width = (width.saturating_sub(GUTTER_WIDTH * 2 + 1)) / 2;
    
    let (left_bg, right_bg, left_fg, right_fg) = match line.kind {
        DiffLineKind::Context => (
            palette::BG_PANEL,
            palette::BG_PANEL,
            palette::TEXT_SECONDARY,
            palette::TEXT_SECONDARY,
        ),
        DiffLineKind::Added => (
            palette::BG_PANEL,
            Color::Rgb(20, 60, 30),
            palette::TEXT_DIM,
            palette::GREEN,
        ),
        DiffLineKind::Removed => (
            Color::Rgb(60, 20, 30),
            palette::BG_PANEL,
            palette::PINK,
            palette::TEXT_DIM,
        ),
        DiffLineKind::Modified => (
            Color::Rgb(60, 40, 20),
            Color::Rgb(20, 60, 30),
            palette::WARNING,
            palette::GREEN,
        ),
    };

    let border_color = if selected { palette::CYAN } else { palette::BORDER_DIM };

    let left_num = line.left_num
        .map(|n| format!("{:>4}", n))
        .unwrap_or_else(|| "    ".to_string());
    
    let right_num = line.right_num
        .map(|n| format!("{:>4}", n))
        .unwrap_or_else(|| "    ".to_string());

    let left_text = truncate_or_pad(&line.left_content, half_width as usize);
    let right_text = truncate_or_pad(&line.right_content, half_width as usize);

    Line::from(vec![
        Span::styled(left_num, Style::default().fg(palette::TEXT_DIM).bg(left_bg)),
        Span::styled(" ", Style::default().bg(left_bg)),
        Span::styled(left_text, Style::default().fg(left_fg).bg(left_bg)),
        Span::styled("\u{2502}", Style::default().fg(border_color)),
        Span::styled(right_num, Style::default().fg(palette::TEXT_DIM).bg(right_bg)),
        Span::styled(" ", Style::default().bg(right_bg)),
        Span::styled(right_text, Style::default().fg(right_fg).bg(right_bg)),
    ])
}

pub fn render_hunk_header(hunk: &DiffHunk, width: u16, selected: bool) -> Line<'static> {
    let status_icon = match hunk.accepted {
        Some(true) => ("\u{F00C0}", palette::GREEN),
        Some(false) => ("\u{F0159}", palette::PINK),
        None => ("\u{F0142}", palette::WARNING),
    };

    let header = format!(
        " {} @@ -{},{} +{},{} @@",
        status_icon.0,
        hunk.left_start,
        hunk.lines.len(),
        hunk.right_start,
        hunk.lines.len()
    );

    let bg = if selected { palette::BG_ACTIVE } else { palette::BG_ELEVATED };
    let padded = truncate_or_pad(&header, width as usize);

    Line::from(vec![
        Span::styled(padded, Style::default().fg(status_icon.1).bg(bg))
    ])
}

fn truncate_or_pad(s: &str, width: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > width {
        let mut result: String = chars[..width.saturating_sub(1)].iter().collect();
        result.push('\u{2026}');
        result
    } else {
        let mut result = s.to_string();
        result.push_str(&" ".repeat(width.saturating_sub(chars.len())));
        result
    }
}
