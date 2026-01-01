use ratatui::widgets::ListState;

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
