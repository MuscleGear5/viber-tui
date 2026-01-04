use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use petgraph::visit::EdgeRef;

/// Represents a single task in the DAG
#[derive(Debug, Clone)]
pub struct TaskNode {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub dependencies: Vec<String>,
}

impl TaskNode {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            status: TaskStatus::Pending,
            dependencies: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }
}

/// Execution status of a task
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Ready,
    Running,
    Completed,
    Failed,
    Skipped,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Ready => write!(f, "Ready"),
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
            TaskStatus::Skipped => write!(f, "Skipped"),
        }
    }
}

/// DAG-based task graph for parallel execution
#[derive(Debug)]
pub struct TaskGraph {
    graph: DiGraph<TaskNode, ()>,
    node_map: HashMap<String, NodeIndex>,
}

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Cycle detected in dependency graph")]
    CycleDetected,

    #[error("Failed to add dependency edge: {0}")]
    EdgeError(String),
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Add a task node to the graph
    pub fn add_node(&mut self, task: TaskNode) -> Result<(), GraphError> {
        if self.node_map.contains_key(&task.id) {
            return Err(GraphError::EdgeError(format!(
                "Task {} already exists",
                task.id
            )));
        }

        let node_idx = self.graph.add_node(task.clone());
        self.node_map.insert(task.id, node_idx);
        Ok(())
    }

    /// Add a dependency edge between two tasks
    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<(), GraphError> {
        let from_idx = self.node_map.get(from)
            .ok_or_else(|| GraphError::TaskNotFound(from.to_string()))?;

        let to_idx = self.node_map.get(to)
            .ok_or_else(|| GraphError::TaskNotFound(to.to_string()))?;

        self.graph.add_edge(*from_idx, *to_idx, ());
        Ok(())
    }

    /// Get all tasks that are ready to execute (no unmet dependencies)
    pub fn get_ready_nodes(&self) -> Vec<&TaskNode> {
        let mut ready = Vec::new();

        for (node_idx, node) in self.graph.node_indices().zip(self.graph.node_weights()) {
            if node.status == TaskStatus::Pending {
                // Check if all dependencies are completed
                let deps_completed = self
                    .graph
                    .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                    .all(|neighbor_idx| {
                        self.graph[neighbor_idx].status == TaskStatus::Completed
                    });

                if deps_completed {
                    ready.push(node);
                }
            }
        }

        ready
    }

    /// Mark a task as completed
    pub fn mark_complete(&mut self, task_id: &str) -> Result<(), GraphError> {
        let node_idx = self.node_map.get(task_id)
            .ok_or_else(|| GraphError::TaskNotFound(task_id.to_string()))?;

        self.graph[*node_idx].status = TaskStatus::Completed;
        Ok(())
    }

    /// Mark a task as failed
    pub fn mark_failed(&mut self, task_id: &str, reason: Option<String>) -> Result<(), GraphError> {
        let node_idx = self.node_map.get(task_id)
            .ok_or_else(|| GraphError::TaskNotFound(task_id.to_string()))?;

        self.graph[*node_idx].status = TaskStatus::Failed;
        Ok(())
    }

    /// Mark a task as running
    pub fn mark_running(&mut self, task_id: &str) -> Result<(), GraphError> {
        let node_idx = self.node_map.get(task_id)
            .ok_or_else(|| GraphError::TaskNotFound(task_id.to_string()))?;

        self.graph[*node_idx].status = TaskStatus::Running;
        Ok(())
    }

    /// Get topological sort of all tasks (execution order)
    pub fn topological_sort(&self) -> Result<Vec<NodeIndex>, GraphError> {
        match toposort(&self.graph, None) {
            Ok(sorted) => Ok(sorted),
            Err(_) => Err(GraphError::CycleDetected),
        }
    }

    /// Get all nodes
    pub fn nodes(&self) -> Vec<&TaskNode> {
        self.graph.node_weights().collect()
    }

    /// Get a specific node by ID
    pub fn get_node(&self, task_id: &str) -> Option<&TaskNode> {
        self.node_map.get(task_id).map(|idx| &self.graph[*idx])
    }

    /// Check if all tasks are completed
    pub fn is_complete(&self) -> bool {
        self.graph
            .node_weights()
            .all(|node| node.status == TaskStatus::Completed)
    }

    /// Get pending task count
    pub fn pending_count(&self) -> usize {
        self.graph
            .node_weights()
            .filter(|node| node.status == TaskStatus::Pending)
            .count()
    }

    /// Get completed task count
    pub fn completed_count(&self) -> usize {
        self.graph
            .node_weights()
            .filter(|node| node.status == TaskStatus::Completed)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph = TaskGraph::new();
        assert_eq!(graph.pending_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = TaskGraph::new();
        let task = TaskNode::new("task1".to_string(), "Task 1".to_string());
        assert!(graph.add_node(task.clone()).is_ok());
        assert_eq!(graph.nodes().len(), 1);
        assert_eq!(graph.get_node("task1").unwrap().id, "task1");
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("task1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("task2".to_string(), "Task 2".to_string());
        graph.add_node(task1).unwrap();
        graph.add_node(task2).unwrap();
        assert!(graph.add_dependency("task1", "task2").is_ok());
    }

    #[test]
    fn test_get_ready_nodes() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("task1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("task2".to_string(), "Task 2".to_string());
        graph.add_node(task1.clone()).unwrap();
        graph.add_node(task2.clone()).unwrap();

        let ready = graph.get_ready_nodes();
        assert_eq!(ready.len(), 2);

        // Add dependency: task2 depends on task1
        graph.add_dependency("task1", "task2").unwrap();
        let ready = graph.get_ready_nodes();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "task1");
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("task1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("task2".to_string(), "Task 2".to_string());
        let task3 = TaskNode::new("task3".to_string(), "Task 3".to_string());

        graph.add_node(task1).unwrap();
        graph.add_node(task2).unwrap();
        graph.add_node(task3).unwrap();

        graph.add_dependency("task1", "task2").unwrap();
        graph.add_dependency("task2", "task3").unwrap();

        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted.len(), 3);
        // task1 should come before task2, which should come before task3
        let ids: Vec<_> = sorted.iter().map(|idx| &graph.graph[*idx].id).collect();
        let pos1 = ids.iter().position(|id| *id == "task1").unwrap();
        let pos2 = ids.iter().position(|id| *id == "task2").unwrap();
        let pos3 = ids.iter().position(|id| *id == "task3").unwrap();
        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("task1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("task2".to_string(), "Task 2".to_string());

        graph.add_node(task1).unwrap();
        graph.add_node(task2).unwrap();

        graph.add_dependency("task1", "task2").unwrap();
        graph.add_dependency("task2", "task1").unwrap();

        assert!(graph.topological_sort().is_err());
    }

    #[test]
    fn test_mark_complete() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("task1".to_string(), "Task 1".to_string());
        graph.add_node(task1).unwrap();

        graph.mark_complete("task1").unwrap();
        assert_eq!(graph.get_node("task1").unwrap().status, TaskStatus::Completed);
        assert_eq!(graph.completed_count(), 1);
    }

    #[test]
    fn test_is_complete() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("task1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("task2".to_string(), "Task 2".to_string());

        graph.add_node(task1).unwrap();
        graph.add_node(task2).unwrap();

        assert!(!graph.is_complete());

        graph.mark_complete("task1").unwrap();
        assert!(!graph.is_complete());

        graph.mark_complete("task2").unwrap();
        assert!(graph.is_complete());
    }
}
