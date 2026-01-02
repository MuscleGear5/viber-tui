use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: Instant,
    pub is_streaming: bool,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            timestamp: Instant::now(),
            is_streaming: false,
        }
    }

    pub fn streaming(role: MessageRole) -> Self {
        Self {
            role,
            content: String::new(),
            timestamp: Instant::now(),
            is_streaming: true,
        }
    }

    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn finish_streaming(&mut self) {
        self.is_streaming = false;
    }
}

#[derive(Debug)]
pub struct ChatState {
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub cursor_pos: usize,
    pub scroll_offset: usize,
    pub input_focused: bool,
    pub input_history: Vec<String>,
    pub history_index: Option<usize>,
    pub stream_reveal_pos: usize,
}

impl Default for ChatState {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            cursor_pos: 0,
            scroll_offset: 0,
            input_focused: true,
            input_history: Vec::new(),
            history_index: None,
            stream_reveal_pos: 0,
        }
    }

    pub fn push_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
        self.scroll_to_bottom();
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.messages.len().saturating_sub(1);
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub fn scroll_down(&mut self, lines: usize) {
        let max = self.messages.len().saturating_sub(1);
        self.scroll_offset = (self.scroll_offset + lines).min(max);
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            let prev = self.input[..self.cursor_pos]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor_pos -= prev;
            self.input.remove(self.cursor_pos);
        }
    }

    pub fn take_input(&mut self) -> String {
        self.cursor_pos = 0;
        std::mem::take(&mut self.input)
    }

    pub fn current_streaming_message(&mut self) -> Option<&mut ChatMessage> {
        self.messages.iter_mut().rev().find(|m| m.is_streaming)
    }

    pub fn history_up(&mut self) {
        if self.input_history.is_empty() {
            return;
        }
        let new_idx = match self.history_index {
            None => self.input_history.len() - 1,
            Some(i) => i.saturating_sub(1),
        };
        self.history_index = Some(new_idx);
        self.input = self.input_history[new_idx].clone();
        self.cursor_pos = self.input.len();
    }

    pub fn history_down(&mut self) {
        let Some(idx) = self.history_index else { return };
        if idx + 1 >= self.input_history.len() {
            self.history_index = None;
            self.input.clear();
            self.cursor_pos = 0;
        } else {
            self.history_index = Some(idx + 1);
            self.input = self.input_history[idx + 1].clone();
            self.cursor_pos = self.input.len();
        }
    }

    pub fn submit_input(&mut self) -> Option<String> {
        let text = self.take_input();
        if text.is_empty() {
            return None;
        }
        self.input_history.push(text.clone());
        self.history_index = None;
        Some(text)
    }

    pub fn advance_stream_reveal(&mut self) {
        if let Some(msg) = self.messages.iter().rev().find(|m| m.is_streaming) {
            let content_len = msg.content.len();
            if self.stream_reveal_pos < content_len {
                self.stream_reveal_pos += 1;
            }
        }
    }

    pub fn revealed_content(&self) -> Option<&str> {
        self.messages
            .iter()
            .rev()
            .find(|m| m.is_streaming)
            .map(|m| &m.content[..self.stream_reveal_pos.min(m.content.len())])
    }

    pub fn reset_stream_reveal(&mut self) {
        self.stream_reveal_pos = 0;
    }
}
