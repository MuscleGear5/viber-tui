use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvimConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl NvimConnectionState {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Disconnected => "\u{F0312}",
            Self::Connecting => "\u{F110A}",
            Self::Connected => "\u{F0318}",
            Self::Error => "\u{F0159}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct NvimBuffer {
    pub id: u32,
    pub name: String,
    pub line_count: usize,
    pub is_modified: bool,
}

#[derive(Debug, Clone)]
pub struct NvimCursor {
    pub buffer_id: u32,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct LspClient {
    pub name: String,
    pub root_dir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NvimClient {
    pub connection_id: Option<String>,
    pub state: NvimConnectionState,
    pub socket_path: Option<String>,
    pub buffers: HashMap<u32, NvimBuffer>,
    pub cursor: Option<NvimCursor>,
    pub lsp_clients: Vec<LspClient>,
}

impl NvimClient {
    pub fn new() -> Self {
        Self {
            connection_id: None,
            state: NvimConnectionState::Disconnected,
            socket_path: None,
            buffers: HashMap::new(),
            cursor: None,
            lsp_clients: Vec::new(),
        }
    }

    pub fn connect(&mut self, socket_path: &str, connection_id: &str) {
        self.socket_path = Some(socket_path.to_string());
        self.connection_id = Some(connection_id.to_string());
        self.state = NvimConnectionState::Connected;
    }

    pub fn disconnect(&mut self) {
        self.connection_id = None;
        self.state = NvimConnectionState::Disconnected;
        self.buffers.clear();
        self.cursor = None;
        self.lsp_clients.clear();
    }

    pub fn set_error(&mut self) {
        self.state = NvimConnectionState::Error;
    }

    pub fn is_connected(&self) -> bool {
        self.state == NvimConnectionState::Connected && self.connection_id.is_some()
    }

    pub fn update_buffers(&mut self, buffers: Vec<NvimBuffer>) {
        self.buffers.clear();
        for buf in buffers {
            self.buffers.insert(buf.id, buf);
        }
    }

    pub fn update_cursor(&mut self, cursor: NvimCursor) {
        self.cursor = Some(cursor);
    }

    pub fn update_lsp_clients(&mut self, clients: Vec<LspClient>) {
        self.lsp_clients = clients;
    }

    pub fn current_buffer(&self) -> Option<&NvimBuffer> {
        self.cursor.as_ref().and_then(|c| self.buffers.get(&c.buffer_id))
    }

    pub fn status_line(&self) -> String {
        match self.state {
            NvimConnectionState::Disconnected => "nvim: disconnected".to_string(),
            NvimConnectionState::Connecting => "nvim: connecting...".to_string(),
            NvimConnectionState::Error => "nvim: error".to_string(),
            NvimConnectionState::Connected => {
                let buf_count = self.buffers.len();
                let lsp_count = self.lsp_clients.len();
                format!("nvim: {} bufs, {} lsp", buf_count, lsp_count)
            }
        }
    }
}

impl Default for NvimClient {
    fn default() -> Self {
        Self::new()
    }
}
