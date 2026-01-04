use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::views::workflow::dag::{TaskGraph, TaskNode, TaskStatus};

/// Parallel execution engine for task graph
#[derive(Debug)]
pub struct TaskExecutor {
    graph: Arc<Mutex<TaskGraph>>,
    max_parallel: usize,
}

impl TaskExecutor {
    pub fn new(graph: TaskGraph, max_parallel: usize) -> Self {
        Self {
            graph: Arc::new(Mutex::new(graph)),
            max_parallel,
        }
    }

    /// Execute all tasks in dependency order, running up to max_parallel in parallel
    pub fn execute(&self) -> Result<(), String> {
        let mut running_tasks: Vec<thread::JoinHandle<()>> = Vec::new();
        let mut completed_tasks: usize = 0;

        loop {
            // Get current graph state
            let (ready_count, total_count) = {
                let graph = self.graph.lock().map_err(|e| format!("Lock error: {}", e))?;
                let ready = graph.get_ready_nodes().len();
                let total = graph.nodes().len();
                (ready, total)
            };

            // Check if we're done
            if completed_tasks == total_count {
                break;
            }

            // Get ready tasks and spawn them
            let ready_tasks = {
                let mut graph = self.graph.lock().map_err(|e| format!("Lock error: {}", e))?;
                let ready = graph.get_ready_nodes();
                for task in &ready {
                    if task.status == TaskStatus::Pending {
                        graph.mark_running(&task.id)?;
                    }
                }
                ready.into_iter().map(|t| t.id.clone()).collect::<Vec<_>>()
            };

            for task_id in ready_tasks {
                if running_tasks.len() < self.max_parallel {
                    let graph_clone = Arc::clone(&self.graph);
                    let handle = thread::spawn(move || {
                        Self::execute_task(task_id.clone(), graph_clone);
                    });
                    running_tasks.push(handle);
                }
            }

            // Wait for at least one task to complete
            if !running_tasks.is_empty() {
                let handle = running_tasks.remove(0);
                handle.join().map_err(|e| format!("Task thread panicked: {:?}", e))?;
                completed_tasks += 1;
            }

            // Small sleep to avoid busy-waiting
            thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }

    fn execute_task(task_id: String, graph: Arc<Mutex<TaskGraph>>) {
        // Mark as complete after execution
        let result = match Self::run_task_logic(&task_id) {
            Ok(_) => {
                let mut g = graph.lock().unwrap();
                g.mark_complete(&task_id)
            }
            Err(e) => {
                let mut g = graph.lock().unwrap();
                g.mark_failed(&task_id, Some(e))
            }
        };

        if let Err(e) = result {
            eprintln!("Error updating task {}: {}", task_id, e);
        }
    }

    /// Placeholder for actual task execution logic
    /// This would be replaced with real task execution based on task type
    fn run_task_logic(task_id: &str) -> Result<(), String> {
        // TODO: Integrate with actual task execution system
        // For now, simulate work
        thread::sleep(Duration::from_millis(100));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let graph = TaskGraph::new();
        let executor = TaskExecutor::new(graph, 4);
        assert_eq!(executor.max_parallel, 4);
    }

    #[test]
    fn test_empty_execution() {
        let graph = TaskGraph::new();
        let executor = TaskExecutor::new(graph, 2);
        assert!(executor.execute().is_ok());
    }

    #[test]
    fn test_simple_parallel_execution() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("t1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("t2".to_string(), "Task 2".to_string());
        let task3 = TaskNode::new("t3".to_string(), "Task 3".to_string());

        graph.add_node(task1).unwrap();
        graph.add_node(task2).unwrap();
        graph.add_node(task3).unwrap();

        let executor = TaskExecutor::new(graph, 2);
        assert!(executor.execute().is_ok());
    }

    #[test]
    fn test_dependency_execution() {
        let mut graph = TaskGraph::new();
        let task1 = TaskNode::new("t1".to_string(), "Task 1".to_string());
        let task2 = TaskNode::new("t2".to_string(), "Task 2".to_string());
        let task3 = TaskNode::new("t3".to_string(), "Task 3".to_string());

        graph.add_node(task1).unwrap();
        graph.add_node(task2).unwrap();
        graph.add_node(task3).unwrap();

        // task2 depends on task1, task3 depends on task2
        graph.add_dependency("t1", "t2").unwrap();
        graph.add_dependency("t2", "t3").unwrap();

        let executor = TaskExecutor::new(graph, 1);
        assert!(executor.execute().is_ok());
    }
}
