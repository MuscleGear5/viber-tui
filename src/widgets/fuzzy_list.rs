use nucleo::{Config, Nucleo, Utf32Str};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};
use std::sync::Arc;

use crate::data::Action;
use crate::theme::colors;

pub struct FuzzyMatcher {
    nucleo: Nucleo<Arc<Action>>,
    last_pattern: String,
}

impl FuzzyMatcher {
    pub fn new() -> Self {
        let nucleo = Nucleo::new(Config::DEFAULT.match_paths(), Arc::new(|| {}), None, 1);
        Self {
            nucleo,
            last_pattern: String::new(),
        }
    }

    pub fn set_actions(&mut self, actions: Vec<Action>) {
        let injector = self.nucleo.injector();
        for action in actions {
            let action = Arc::new(action);
            injector.push(action, |a, cols| {
                cols[0] = a.searchable_text().into();
            });
        }
    }

    pub fn update_pattern(&mut self, pattern: &str) {
        if pattern != self.last_pattern {
            let append = pattern.starts_with(&self.last_pattern);
            self.last_pattern = pattern.to_string();
            self.nucleo.pattern.reparse(
                0,
                pattern,
                nucleo::pattern::CaseMatching::Smart,
                nucleo::pattern::Normalization::Smart,
                append,
            );
        }
    }

    pub fn tick(&mut self) -> bool {
        let status = self.nucleo.tick(10);
        status.changed
    }

    pub fn results(&self) -> Vec<(Arc<Action>, Vec<u32>)> {
        let snapshot = self.nucleo.snapshot();
        snapshot
            .matched_items(..snapshot.matched_item_count().min(100))
            .map(|item| {
                let mut indices = Vec::new();
                let pattern = snapshot.pattern().column_pattern(0);
                if !pattern.atoms.is_empty() {
                    let text = item.data.searchable_text();
                    let mut buf = Vec::new();
                    let haystack = Utf32Str::new(&text, &mut buf);
                    pattern.indices(
                        haystack.slice(..),
                        &mut nucleo::Matcher::default(),
                        &mut indices,
                    );
                }
                (Arc::clone(&item.data), indices)
            })
            .collect()
    }

    pub fn result_count(&self) -> usize {
        self.nucleo.snapshot().matched_item_count() as usize
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FuzzyListState {
    pub list_state: ListState,
    pub scroll_offset: usize,
    pub visible_count: usize,
}

impl FuzzyListState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            scroll_offset: 0,
            visible_count: 10,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.list_state.select(index);
    }

    pub fn select_next(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let next = (current + 1).min(total - 1);
        self.list_state.select(Some(next));
        self.ensure_visible(total);
    }

    pub fn select_previous(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let prev = current.saturating_sub(1);
        self.list_state.select(Some(prev));
        self.ensure_visible(total);
    }

    pub fn select_first(&mut self) {
        self.list_state.select(Some(0));
        self.scroll_offset = 0;
    }

    pub fn select_last(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        self.list_state.select(Some(total - 1));
        self.ensure_visible(total);
    }

    fn ensure_visible(&mut self, _total: usize) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.scroll_offset {
                self.scroll_offset = selected;
            } else if selected >= self.scroll_offset + self.visible_count {
                self.scroll_offset = selected.saturating_sub(self.visible_count - 1);
            }
        }
    }

    pub fn page_down(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let next = (current + self.visible_count).min(total - 1);
        self.list_state.select(Some(next));
        self.ensure_visible(total);
    }

    pub fn page_up(&mut self, total: usize) {
        if total == 0 {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0);
        let prev = current.saturating_sub(self.visible_count);
        self.list_state.select(Some(prev));
        self.ensure_visible(total);
    }
}

impl Default for FuzzyListState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FuzzyList<'a> {
    items: &'a [(Arc<Action>, Vec<u32>)],
    block: Option<Block<'a>>,
}

impl<'a> FuzzyList<'a> {
    pub fn new(items: &'a [(Arc<Action>, Vec<u32>)]) -> Self {
        Self { items, block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    fn render_action_line(action: &Action, match_indices: &[u32]) -> Line<'static> {
        let icon = action.display_icon();
        let cat_color = colors::category_color(action.category);

        let icon_span = Span::styled(format!("{} ", icon), Style::default().fg(cat_color));

        let name = &action.name;
        let mut name_spans = Vec::new();
        let mut last_idx = 0;

        let name_len = name.len();
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
                colors::text_primary()
                    .add_modifier(Modifier::BOLD)
                    .fg(colors::ACCENT_CYAN),
            ));
            last_idx = idx + 1;
        }
        if last_idx < name_len {
            name_spans.push(Span::styled(
                name[last_idx..].to_string(),
                colors::text_primary(),
            ));
        }

        let desc_span = Span::styled(format!("  {}", action.description), colors::text_muted());

        let mut spans = vec![icon_span];
        spans.extend(name_spans);
        spans.push(desc_span);

        Line::from(spans)
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
                let line = Self::render_action_line(action, indices);
                let style = if Some(i) == state.list_state.selected() {
                    colors::list_selected()
                } else {
                    Style::default()
                };
                ListItem::new(line).style(style)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(colors::list_selected())
            .highlight_symbol("▶ ");

        StatefulWidget::render(list, inner_area, buf, &mut state.list_state);
    }
}
