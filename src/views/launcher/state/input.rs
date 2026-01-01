use super::LauncherState;

pub trait InputHandler {
    fn insert_char(&mut self, c: char);
    fn delete_char(&mut self);
    fn delete_char_forward(&mut self);
    fn move_cursor_left(&mut self);
    fn move_cursor_right(&mut self);
    fn move_cursor_start(&mut self);
    fn move_cursor_end(&mut self);
    fn clear_input(&mut self);
}

impl InputHandler for LauncherState {
    fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
        self.on_input_changed();
    }

    fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            let prev = self.input[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(prev);
            self.cursor_pos = prev;
            self.on_input_changed();
        }
    }

    fn delete_char_forward(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.input.remove(self.cursor_pos);
            self.on_input_changed();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos = self.input[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos = self.input[self.cursor_pos..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor_pos + i)
                .unwrap_or(self.input.len());
        }
    }

    fn move_cursor_start(&mut self) {
        self.cursor_pos = 0;
    }

    fn move_cursor_end(&mut self) {
        self.cursor_pos = self.input.len();
    }

    fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_pos = 0;
        self.on_input_changed();
    }
}
