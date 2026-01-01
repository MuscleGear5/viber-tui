use std::collections::HashMap;
use super::models::{DagNode, NodeId, Edge};

pub struct DagState {
    pub nodes: HashMap<NodeId, DagNode>,
    pub selected: Option<NodeId>,
    pub scroll_x: u16,
    pub scroll_y: u16,
}

impl Default for DagState {
    fn default() -> Self { Self::new() }
}

impl DagState {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), selected: None, scroll_x: 0, scroll_y: 0 }
    }

    pub fn add_node(&mut self, node: DagNode) { self.nodes.insert(node.id.clone(), node); }

    pub fn get_node(&self, id: &NodeId) -> Option<&DagNode> { self.nodes.get(id) }

    pub fn edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        for node in self.nodes.values() {
            for dep_id in &node.dependencies {
                if let Some(dep) = self.nodes.get(dep_id) {
                    edges.push(Edge::new((dep.x, dep.y), (node.x, node.y)));
                }
            }
        }
        edges
    }

    pub fn select_next(&mut self) {
        let ids: Vec<_> = self.nodes.keys().cloned().collect();
        if ids.is_empty() { return; }
        self.selected = match &self.selected {
            None => Some(ids[0].clone()),
            Some(curr) => ids.iter()
                .position(|id| id == curr)
                .map(|i| ids[(i + 1) % ids.len()].clone()),
        };
    }

    pub fn select_prev(&mut self) {
        let ids: Vec<_> = self.nodes.keys().cloned().collect();
        if ids.is_empty() { return; }
        self.selected = match &self.selected {
            None => ids.last().cloned(),
            Some(curr) => ids.iter()
                .position(|id| id == curr)
                .map(|i| ids[(i + ids.len() - 1) % ids.len()].clone()),
        };
    }

    pub fn scroll(&mut self, dx: i16, dy: i16) {
        self.scroll_x = self.scroll_x.saturating_add_signed(dx);
        self.scroll_y = self.scroll_y.saturating_add_signed(dy);
    }

    pub fn layout_auto(&mut self) {
        let mut levels: HashMap<NodeId, u16> = HashMap::new();
        for id in self.nodes.keys() {
            self.calc_level(id, &mut levels);
        }
        let mut by_level: HashMap<u16, Vec<NodeId>> = HashMap::new();
        for (id, level) in &levels {
            by_level.entry(*level).or_default().push(id.clone());
        }
        for (level, ids) in by_level {
            for (i, id) in ids.into_iter().enumerate() {
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.x = level * 20;
                    node.y = i as u16 * 4;
                }
            }
        }
    }

    fn calc_level(&self, id: &NodeId, levels: &mut HashMap<NodeId, u16>) -> u16 {
        if let Some(&l) = levels.get(id) { return l; }
        let node = match self.nodes.get(id) { Some(n) => n, None => return 0 };
        let max_dep = node.dependencies.iter()
            .map(|d| self.calc_level(d, levels))
            .max().unwrap_or(0);
        let level = if node.dependencies.is_empty() { 0 } else { max_dep + 1 };
        levels.insert(id.clone(), level);
        level
    }
}
