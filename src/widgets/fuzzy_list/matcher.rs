use nucleo::{Config, Nucleo, Utf32Str};
use std::sync::Arc;

use crate::data::Action;

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
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new()
    }
}
