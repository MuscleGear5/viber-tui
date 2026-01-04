use super::state::Task;

pub struct TasksState {
    pub tasks: Vec<Task>,
    pub filtered_indices: Vec<usize>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub filter_query: String,
}

impl Default for TasksState {
    fn default() -> Self {
        Self::new()
    }
}

impl TasksState {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            filtered_indices: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            filter_query: String::new(),
        }
    }

    pub fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
        self.apply_filter();
    }

    pub fn set_filter(&mut self, query: String) {
        self.filter_query = query;
        self.apply_filter();
    }

    fn apply_filter(&mut self) {
        let query = self.filter_query.to_lowercase();
        self.filtered_indices = self.tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| query.is_empty() || t.title.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect();
        
        self.selected_index = self.selected_index.min(self.filtered_indices.len().saturating_sub(1));
    }

    pub fn select_next(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.filtered_indices.len() - 1);
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn selected_task(&self) -> Option<&Task> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|&i| self.tasks.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::tasks::state::{TaskPriority, TaskStatus};

    #[test]
    fn test_tasks_state_new() {
        let state = TasksState::new();
        assert!(state.tasks.is_empty());
        assert!(state.filter_query.is_empty());
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_set_tasks_and_filter() {
        let mut state = TasksState::new();
        state.set_tasks(vec![
            Task::new("1", "Implement feature"),
            Task::new("2", "Fix bug"),
            Task::new("3", "Implement tests"),
        ]);
        assert_eq!(state.filtered_indices.len(), 3);
        
        state.set_filter("Implement".into());
        assert_eq!(state.filtered_indices.len(), 2);
        assert_eq!(state.filtered_indices, vec![0, 2]);
    }

    #[test]
    fn test_navigation() {
        let mut state = TasksState::new();
        state.set_tasks(vec![
            Task::new("1", "A"),
            Task::new("2", "B"),
            Task::new("3", "C"),
        ]);
        assert_eq!(state.selected_index, 0);
        
        state.select_next();
        assert_eq!(state.selected_index, 1);
        
        state.select_next();
        state.select_next();
        assert_eq!(state.selected_index, 2);
        
        state.select_prev();
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn test_selected_task() {
        let mut state = TasksState::new();
        assert!(state.selected_task().is_none());
        
        state.set_tasks(vec![Task::new("t1", "Task One")]);
        let task = state.selected_task().unwrap();
        assert_eq!(task.id, "t1");
    }
}
