use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct MemcordSlot {
    pub name: String,
    pub entries: Vec<MemcordEntry>,
    pub tags: Vec<String>,
    pub group: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MemcordEntry {
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemcordCommand {
    Name,
    Use,
    Save,
    Read,
    SaveProgress,
    List,
    Search,
    Query,
    Zero,
    Tag,
    Group,
    Archive,
}

impl MemcordCommand {
    pub fn all() -> &'static [Self] {
        &[
            Self::Name, Self::Use, Self::Save, Self::Read, Self::SaveProgress,
            Self::List, Self::Search, Self::Query, Self::Zero,
            Self::Tag, Self::Group, Self::Archive,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Use => "use",
            Self::Save => "save",
            Self::Read => "read",
            Self::SaveProgress => "save_progress",
            Self::List => "list",
            Self::Search => "search",
            Self::Query => "query",
            Self::Zero => "zero",
            Self::Tag => "tag",
            Self::Group => "group",
            Self::Archive => "archive",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::Name => "Create or select memory slot",
            Self::Use => "Switch to existing slot",
            Self::Save => "Save text to current slot",
            Self::Read => "Retrieve memory content",
            Self::SaveProgress => "Auto-summarize with timestamp",
            Self::List => "List all memory slots",
            Self::Search => "Full-text search",
            Self::Query => "Natural language query",
            Self::Zero => "Privacy mode - block saving",
            Self::Tag => "Add/remove tags",
            Self::Group => "Organize into folders",
            Self::Archive => "Long-term storage",
        }
    }
}

#[derive(Debug, Default)]
pub struct MemcordState {
    pub slots: HashMap<String, MemcordSlot>,
    pub current_slot: Option<String>,
    pub zero_mode: bool,
    pub last_error: Option<String>,
}

impl MemcordState {
    pub fn new() -> Self { Self::default() }
    
    pub fn select_slot(&mut self, name: &str) {
        if !self.slots.contains_key(name) {
            self.slots.insert(name.to_string(), MemcordSlot {
                name: name.to_string(),
                ..Default::default()
            });
        }
        self.current_slot = Some(name.to_string());
    }
    
    pub fn current(&self) -> Option<&MemcordSlot> {
        self.current_slot.as_ref().and_then(|n| self.slots.get(n))
    }
    
    pub fn save(&mut self, content: String, timestamp: u64) -> bool {
        if self.zero_mode { return false; }
        if let Some(name) = &self.current_slot {
            if let Some(slot) = self.slots.get_mut(name) {
                slot.entries.push(MemcordEntry { content, timestamp });
                return true;
            }
        }
        false
    }
}
