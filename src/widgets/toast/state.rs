use super::models::{ToastAction, ToastLevel, ToastNotification};

#[derive(Debug, Default)]
pub struct ToastState {
    toasts: Vec<ToastNotification>,
    next_id: u64,
    selected_action: usize,
}

impl ToastState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, message: impl Into<String>, level: ToastLevel) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(ToastNotification::new(id, message, level));
        self.selected_action = 0;
        id
    }

    pub fn push_toast(&mut self, mut toast: ToastNotification) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        toast.id = id;
        self.toasts.push(toast);
        self.selected_action = 0;
        id
    }

    pub fn info(&mut self, msg: impl Into<String>) -> u64 { self.push(msg, ToastLevel::Info) }
    pub fn success(&mut self, msg: impl Into<String>) -> u64 { self.push(msg, ToastLevel::Success) }
    pub fn warning(&mut self, msg: impl Into<String>) -> u64 { self.push(msg, ToastLevel::Warning) }
    pub fn error(&mut self, msg: impl Into<String>) -> u64 { self.push(msg, ToastLevel::Error) }

    pub fn tick(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
        if let Some(toast) = self.active() {
            if self.selected_action >= toast.actions.len().max(1) {
                self.selected_action = 0;
            }
        }
    }

    pub fn active(&self) -> Option<&ToastNotification> {
        self.toasts.first()
    }

    pub fn dismiss(&mut self) {
        if !self.toasts.is_empty() {
            self.toasts.remove(0);
            self.selected_action = 0;
        }
    }

    pub fn dismiss_id(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }

    pub fn next_action(&mut self) {
        if let Some(toast) = self.active() {
            let count = toast.actions.len().max(1);
            self.selected_action = (self.selected_action + 1) % count;
        }
    }

    pub fn prev_action(&mut self) {
        if let Some(toast) = self.active() {
            let count = toast.actions.len().max(1);
            self.selected_action = self.selected_action.checked_sub(1).unwrap_or(count - 1);
        }
    }

    pub fn selected(&self) -> Option<&ToastAction> {
        self.active().and_then(|t| t.actions.get(self.selected_action))
    }

    pub fn selected_index(&self) -> usize { self.selected_action }
    pub fn count(&self) -> usize { self.toasts.len() }
    pub fn is_empty(&self) -> bool { self.toasts.is_empty() }
}
