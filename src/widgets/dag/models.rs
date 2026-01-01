use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus { Pending, InProgress, Complete, Failed, Blocked }

impl NodeStatus {
    pub fn color(&self) -> Color {
        match self {
            Self::Pending => Color::Rgb(107, 114, 128),
            Self::InProgress => Color::Rgb(0, 255, 255),
            Self::Complete => Color::Rgb(16, 185, 129),
            Self::Failed => Color::Rgb(255, 107, 151),
            Self::Blocked => Color::Rgb(245, 158, 11),
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Pending => "\u{F0453}",
            Self::InProgress => "\u{F0150}",
            Self::Complete => "\u{F00C0}",
            Self::Failed => "\u{F0159}",
            Self::Blocked => "\u{F033E}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DagNode {
    pub id: NodeId,
    pub label: String,
    pub status: NodeStatus,
    pub dependencies: Vec<NodeId>,
    pub x: u16,
    pub y: u16,
}

impl DagNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: NodeId::new(id),
            label: label.into(),
            status: NodeStatus::Pending,
            dependencies: Vec::new(),
            x: 0, y: 0,
        }
    }
    pub fn with_status(mut self, status: NodeStatus) -> Self { self.status = status; self }
    pub fn with_deps(mut self, deps: Vec<NodeId>) -> Self { self.dependencies = deps; self }
    pub fn at(mut self, x: u16, y: u16) -> Self { self.x = x; self.y = y; self }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge { pub from_x: u16, pub from_y: u16, pub to_x: u16, pub to_y: u16 }

impl Edge {
    pub fn new(from: (u16, u16), to: (u16, u16)) -> Self {
        Self { from_x: from.0, from_y: from.1, to_x: to.0, to_y: to.1 }
    }
}
