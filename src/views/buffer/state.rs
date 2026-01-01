use super::language::Language;
use super::nvim::{NvimConnection, NvimSyncStatus};

#[derive(Debug, Clone)]
pub struct BufferLine {
    pub number: usize,
    pub content: String,
    pub is_modified: bool,
    pub has_diagnostic: bool,
}

impl BufferLine {
    pub fn new(number: usize, content: String) -> Self {
        Self {
            number,
            content,
            is_modified: false,
            has_diagnostic: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferState {
    pub path: Option<String>,
    pub language: Language,
    pub lines: Vec<BufferLine>,
    pub scroll_offset: usize,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_modified: bool,
    pub nvim: NvimConnection,
}

impl Default for BufferState {
    fn default() -> Self {
        Self {
            path: None,
            language: Language::Plain,
            lines: vec![BufferLine::new(1, String::new())],
            scroll_offset: 0,
            cursor_line: 0,
            cursor_col: 0,
            is_modified: false,
            nvim: NvimConnection::default(),
        }
    }
}

impl BufferState {
    pub fn from_content(path: &str, content: &str) -> Self {
        let ext = path.rsplit('.').next().unwrap_or("");
        let language = Language::from_extension(ext);
        let lines: Vec<_> = content
            .lines()
            .enumerate()
            .map(|(i, line)| BufferLine::new(i + 1, line.to_string()))
            .collect();
        
        Self {
            path: Some(path.to_string()),
            language,
            lines: if lines.is_empty() { vec![BufferLine::new(1, String::new())] } else { lines },
            scroll_offset: 0,
            cursor_line: 0,
            cursor_col: 0,
            is_modified: false,
            nvim: NvimConnection::default(),
        }
    }
    
    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }
    
    pub fn scroll_down(&mut self, amount: usize, visible_height: usize) {
        let max_scroll = self.lines.len().saturating_sub(visible_height);
        self.scroll_offset = (self.scroll_offset + amount).min(max_scroll);
    }
    
    pub fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            if self.cursor_line < self.scroll_offset {
                self.scroll_offset = self.cursor_line;
            }
        }
    }
    
    pub fn move_cursor_down(&mut self, visible_height: usize) {
        if self.cursor_line < self.lines.len().saturating_sub(1) {
            self.cursor_line += 1;
            if self.cursor_line >= self.scroll_offset + visible_height {
                self.scroll_offset = self.cursor_line - visible_height + 1;
            }
        }
    }
    
    pub fn goto_line(&mut self, line: usize, visible_height: usize) {
        self.cursor_line = line.min(self.lines.len().saturating_sub(1));
        if self.cursor_line < self.scroll_offset {
            self.scroll_offset = self.cursor_line;
        } else if self.cursor_line >= self.scroll_offset + visible_height {
            self.scroll_offset = self.cursor_line - visible_height / 2;
        }
    }
    
    pub fn connect_nvim(&mut self, connection_id: String, buffer_id: u32) {
        self.nvim = NvimConnection::connected(connection_id, buffer_id);
    }
    
    pub fn disconnect_nvim(&mut self) {
        self.nvim = NvimConnection::default();
    }
    
    pub fn sync_from_nvim(&mut self, content: &str) {
        let lines: Vec<_> = content
            .lines()
            .enumerate()
            .map(|(i, line)| BufferLine::new(i + 1, line.to_string()))
            .collect();
        self.lines = if lines.is_empty() { vec![BufferLine::new(1, String::new())] } else { lines };
        self.nvim.sync_status = NvimSyncStatus::Synced;
        self.is_modified = false;
    }
}
