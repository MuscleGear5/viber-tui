#[derive(Debug, Clone, PartialEq)]
pub enum NvimSyncStatus {
    Disconnected,
    Connecting,
    Synced,
    Stale,
}

impl Default for NvimSyncStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[derive(Debug, Clone, Default)]
pub struct NvimConnection {
    pub connection_id: Option<String>,
    pub buffer_id: Option<u32>,
    pub sync_status: NvimSyncStatus,
}

impl NvimConnection {
    pub fn connected(connection_id: String, buffer_id: u32) -> Self {
        Self {
            connection_id: Some(connection_id),
            buffer_id: Some(buffer_id),
            sync_status: NvimSyncStatus::Synced,
        }
    }
    
    pub fn is_connected(&self) -> bool {
        self.connection_id.is_some() && self.sync_status != NvimSyncStatus::Disconnected
    }
}
