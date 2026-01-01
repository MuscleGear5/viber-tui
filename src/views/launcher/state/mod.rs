use crate::data::Action;
use crate::widgets::{FuzzyListState, FuzzyMatcher};
use std::sync::Arc;

mod input;
pub use input::InputHandler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPanel {
    Input,
    List,
    Preview,
}

pub struct LauncherState {
    pub input: String,
    pub cursor_pos: usize,
    pub fuzzy_matcher: FuzzyMatcher,
    pub list_state: FuzzyListState,
    pub results: Vec<(Arc<Action>, Vec<u32>)>,
    pub show_preview: bool,
    pub focus: FocusPanel,
}

impl LauncherState {
    pub fn new(registry: &crate::data::ActionRegistry) -> Self {
        let mut fuzzy_matcher = FuzzyMatcher::new();
        fuzzy_matcher.set_actions(registry.iter().cloned().collect());

        let mut state = Self {
            input: String::new(),
            cursor_pos: 0,
            fuzzy_matcher,
            list_state: FuzzyListState::new(),
            results: Vec::new(),
            show_preview: true,
            focus: FocusPanel::Input,
        };
        state.refresh_results();
        state
    }

    pub fn cycle_focus(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Input => FocusPanel::List,
            FocusPanel::List if self.show_preview => FocusPanel::Preview,
            FocusPanel::List => FocusPanel::Input,
            FocusPanel::Preview => FocusPanel::Input,
        };
    }

    pub fn cycle_focus_reverse(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Input if self.show_preview => FocusPanel::Preview,
            FocusPanel::Input => FocusPanel::List,
            FocusPanel::List => FocusPanel::Input,
            FocusPanel::Preview => FocusPanel::List,
        };
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next(self.results.len());
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous(self.results.len());
    }

    pub fn select_first(&mut self) {
        self.list_state.select_first();
    }

    pub fn select_last(&mut self) {
        self.list_state.select_last(self.results.len());
    }

    pub fn page_down(&mut self) {
        self.list_state.page_down(self.results.len());
    }

    pub fn page_up(&mut self) {
        self.list_state.page_up(self.results.len());
    }

    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
        if !self.show_preview && self.focus == FocusPanel::Preview {
            self.focus = FocusPanel::List;
        }
    }

    pub fn selected_action(&self) -> Option<&Action> {
        self.list_state
            .selected()
            .and_then(|i| self.results.get(i))
            .map(|(action, _)| action.as_ref())
    }

    pub fn tick(&mut self) {
        if self.fuzzy_matcher.tick() {
            self.refresh_results();
        }
    }

    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    pub fn on_input_changed(&mut self) {
        self.fuzzy_matcher.update_pattern(&self.input);
        self.list_state.select_first();
    }

    fn refresh_results(&mut self) {
        self.results = self.fuzzy_matcher.results();
        if self
            .list_state
            .selected()
            .map(|i| i >= self.results.len())
            .unwrap_or(false)
        {
            self.list_state.select_first();
        }
    }
}
