use crate::integrations::nvim::NvimBuffer;

pub struct BufferListState {
    pub buffers: Vec<NvimBuffer>,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl Default for BufferListState {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferListState {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
        }
    }

    pub fn set_buffers(&mut self, buffers: Vec<NvimBuffer>) {
        self.buffers = buffers;
        self.selected_index = self.selected_index.min(self.buffers.len().saturating_sub(1));
    }

    pub fn select_next(&mut self) {
        if !self.buffers.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.buffers.len() - 1);
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn selected_buffer(&self) -> Option<&NvimBuffer> {
        self.buffers.get(self.selected_index)
    }

    pub fn selected_buffer_id(&self) -> Option<u32> {
        self.selected_buffer().map(|b| b.id)
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        }
    }
}
