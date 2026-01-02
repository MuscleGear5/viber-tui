use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Instant;

use super::AgentId;

#[derive(Debug, Clone)]
pub enum ChangeKind {
    FileCreated,
    FileModified,
    FileDeleted,
    DirectoryCreated,
    CommandExecuted,
}

#[derive(Debug, Clone)]
pub struct FileSnapshot {
    pub path: PathBuf,
    pub content: Option<String>,
    pub existed: bool,
}

#[derive(Debug, Clone)]
pub struct Change {
    pub kind: ChangeKind,
    pub path: PathBuf,
    pub before: Option<FileSnapshot>,
    pub agent_id: AgentId,
    pub timestamp: Instant,
    pub description: String,
}

impl Change {
    fn new(kind: ChangeKind, path: PathBuf, before: Option<FileSnapshot>, agent_id: AgentId, desc: impl Into<String>) -> Self {
        Self { kind, path, before, agent_id, timestamp: Instant::now(), description: desc.into() }
    }

    pub fn file_modified(path: PathBuf, content: String, agent_id: AgentId, desc: impl Into<String>) -> Self {
        let snap = FileSnapshot { path: path.clone(), content: Some(content), existed: true };
        Self::new(ChangeKind::FileModified, path, Some(snap), agent_id, desc)
    }

    pub fn file_created(path: PathBuf, agent_id: AgentId, desc: impl Into<String>) -> Self {
        let snap = FileSnapshot { path: path.clone(), content: None, existed: false };
        Self::new(ChangeKind::FileCreated, path, Some(snap), agent_id, desc)
    }

    pub fn file_deleted(path: PathBuf, content: String, agent_id: AgentId, desc: impl Into<String>) -> Self {
        let snap = FileSnapshot { path: path.clone(), content: Some(content), existed: true };
        Self::new(ChangeKind::FileDeleted, path, Some(snap), agent_id, desc)
    }
}

#[derive(Debug)]
pub struct Checkpoint {
    pub id: u64,
    pub changes: Vec<Change>,
    pub description: String,
    pub timestamp: Instant,
}

#[derive(Debug)]
pub struct UndoStack {
    checkpoints: VecDeque<Checkpoint>,
    max_checkpoints: usize,
    next_id: u64,
    pending_changes: Vec<Change>,
}

impl UndoStack {
    pub fn new(max_checkpoints: usize) -> Self {
        Self {
            checkpoints: VecDeque::with_capacity(max_checkpoints),
            max_checkpoints,
            next_id: 1,
            pending_changes: Vec::new(),
        }
    }

    pub fn record(&mut self, change: Change) { self.pending_changes.push(change); }

    pub fn commit(&mut self, description: impl Into<String>) -> u64 {
        if self.pending_changes.is_empty() { return 0; }
        let id = self.next_id;
        self.next_id += 1;
        let checkpoint = Checkpoint {
            id,
            changes: std::mem::take(&mut self.pending_changes),
            description: description.into(),
            timestamp: Instant::now(),
        };
        if self.checkpoints.len() >= self.max_checkpoints { self.checkpoints.pop_front(); }
        self.checkpoints.push_back(checkpoint);
        id
    }

    pub fn pop(&mut self) -> Option<Checkpoint> { self.checkpoints.pop_back() }
    pub fn peek(&self) -> Option<&Checkpoint> { self.checkpoints.back() }
    pub fn len(&self) -> usize { self.checkpoints.len() }
    pub fn is_empty(&self) -> bool { self.checkpoints.is_empty() }
    pub fn clear(&mut self) { self.checkpoints.clear(); self.pending_changes.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_stack_operations() {
        let mut stack = UndoStack::new(2);
        assert!(stack.is_empty());
        stack.record(Change::file_created(PathBuf::from("a.rs"), AgentId(1), "a"));
        assert_eq!(stack.commit("first"), 1);
        stack.record(Change::file_created(PathBuf::from("b.rs"), AgentId(1), "b"));
        stack.commit("second");
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop().expect("cp").description, "second");
    }

    #[test]
    fn undo_stack_max_limit() {
        let mut stack = UndoStack::new(2);
        for i in 0..5 {
            stack.record(Change::file_created(PathBuf::from(format!("{i}.rs")), AgentId(1), ""));
            stack.commit(format!("cp{i}"));
        }
        assert_eq!(stack.len(), 2);
    }
}
