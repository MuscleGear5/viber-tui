use super::models::Modal;
use std::collections::VecDeque;

pub struct ModalState {
    stack: VecDeque<Modal>,
    selected_button: usize,
    next_id: u64,
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            selected_button: 0,
            next_id: 1,
        }
    }

    pub fn push(&mut self, mut modal: Modal) -> u64 {
        modal.id = self.next_id;
        self.next_id += 1;
        self.selected_button = modal.buttons.len().saturating_sub(1);
        self.stack.push_back(modal);
        self.next_id - 1
    }

    pub fn active(&self) -> Option<&Modal> {
        self.stack.back()
    }

    pub fn active_mut(&mut self) -> Option<&mut Modal> {
        self.stack.back_mut()
    }

    pub fn dismiss(&mut self) -> Option<Modal> {
        self.stack.pop_back()
    }

    pub fn dismiss_by_id(&mut self, id: u64) -> Option<Modal> {
        if let Some(pos) = self.stack.iter().position(|m| m.id == id) {
            self.stack.remove(pos)
        } else {
            None
        }
    }

    pub fn has_modal(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn count(&self) -> usize {
        self.stack.len()
    }

    pub fn selected_button(&self) -> usize {
        self.selected_button
    }

    pub fn select_next(&mut self) {
        if let Some(modal) = self.active() {
            if !modal.buttons.is_empty() {
                self.selected_button = (self.selected_button + 1) % modal.buttons.len();
            }
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(modal) = self.active() {
            if !modal.buttons.is_empty() {
                self.selected_button = self.selected_button
                    .checked_sub(1)
                    .unwrap_or(modal.buttons.len() - 1);
            }
        }
    }

    pub fn selected_key(&self) -> Option<char> {
        self.active()
            .and_then(|m| m.buttons.get(self.selected_button))
            .map(|b| b.key)
    }

    pub fn handle_key(&mut self, key: char) -> Option<char> {
        let modal = self.active()?;
        let found = modal.buttons.iter().find(|b| b.key == key)?;
        let result_key = found.key;
        self.dismiss();
        Some(result_key)
    }

    pub fn input_char(&mut self, ch: char) {
        if let Some(modal) = self.active_mut() {
            if let Some(ref mut input) = modal.input_value {
                input.push(ch);
            }
        }
    }

    pub fn input_backspace(&mut self) {
        if let Some(modal) = self.active_mut() {
            if let Some(ref mut input) = modal.input_value {
                input.pop();
            }
        }
    }

    pub fn input_value(&self) -> Option<&str> {
        self.active().and_then(|m| m.input_value.as_deref())
    }
}
