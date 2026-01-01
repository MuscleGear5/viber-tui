use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, List, ListItem, StatefulWidget, Widget},
};
use std::sync::Arc;

use super::render::render_action_line;
use super::FuzzyListState;
use crate::data::Action;
use crate::theme::palette;

pub struct FuzzyList<'a> {
    items: &'a [(Arc<Action>, Vec<u32>)],
    block: Option<Block<'a>>,
    focused: bool,
    glow: f32,
}

impl<'a> FuzzyList<'a> {
    pub fn new(items: &'a [(Arc<Action>, Vec<u32>)]) -> Self {
        Self {
            items,
            block: None,
            focused: false,
            glow: 0.0,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn glow(mut self, glow: f32) -> Self {
        self.glow = glow;
        self
    }
}

impl<'a> StatefulWidget for FuzzyList<'a> {
    type State = FuzzyListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let inner_area = match &self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.clone().render(area, buf);
                inner
            }
            None => area,
        };

        state.visible_count = inner_area.height as usize;

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, (action, indices))| {
                let is_selected = Some(i) == state.list_state.selected();
                let line =
                    render_action_line(action, indices, is_selected, self.focused, self.glow);
                let style = if is_selected {
                    if self.focused {
                        Style::default()
                            .bg(palette::BG_ACTIVE)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().bg(palette::BG_ELEVATED)
                    }
                } else {
                    Style::default()
                };
                ListItem::new(line).style(style)
            })
            .collect();

        let highlight_style = if self.focused {
            Style::default()
                .bg(palette::BG_ACTIVE)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().bg(palette::BG_ELEVATED)
        };

        let highlight_symbol = if self.focused { "> " } else { "  " };

        let list = List::new(items)
            .highlight_style(highlight_style)
            .highlight_symbol(highlight_symbol);

        StatefulWidget::render(list, inner_area, buf, &mut state.list_state);
    }
}
