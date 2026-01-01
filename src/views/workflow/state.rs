use super::models::{Phase, PhaseStatus};

#[derive(Debug)]
pub struct WorkflowState {
    pub phases: Vec<Phase>,
    pub current_phase: Option<usize>,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl Default for WorkflowState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowState {
    pub fn new() -> Self {
        Self {
            phases: Self::viber_phases(),
            current_phase: None,
            selected_index: 0,
            scroll_offset: 0,
        }
    }

    fn viber_phases() -> Vec<Phase> {
        vec![
            Phase::new(0, "Discovery"),
            Phase::new(1, "Requirements").with_deps(vec![0]),
            Phase::new(2, "Spec").with_deps(vec![1]),
            Phase::new(3, "Design").with_deps(vec![2]),
            Phase::new(4, "Implementation").with_deps(vec![3]),
            Phase::new(5, "Testing").with_deps(vec![4]),
            Phase::new(6, "Review").with_deps(vec![5]),
            Phase::new(7, "Integration").with_deps(vec![6]),
            Phase::new(8, "Deployment").with_deps(vec![7]),
        ]
    }

    pub fn overall_progress(&self) -> u8 {
        if self.phases.is_empty() {
            return 0;
        }
        let total: u16 = self.phases.iter().map(|p| p.progress as u16).sum();
        (total / self.phases.len() as u16) as u8
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.phases.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn selected_phase(&self) -> Option<&Phase> {
        self.phases.get(self.selected_index)
    }

    pub fn can_transition_to(&self, phase_id: usize) -> bool {
        let Some(phase) = self.phases.get(phase_id) else {
            return false;
        };
        if phase.status == PhaseStatus::Completed {
            return false;
        }
        phase.dependencies.iter().all(|dep_id| {
            self.phases
                .get(*dep_id)
                .map(|p| p.status == PhaseStatus::Completed)
                .unwrap_or(false)
        })
    }

    pub fn transition_to(&mut self, phase_id: usize) -> bool {
        if !self.can_transition_to(phase_id) {
            return false;
        }
        if let Some(current) = self.current_phase {
            if let Some(phase) = self.phases.get_mut(current) {
                if phase.status == PhaseStatus::InProgress {
                    phase.complete();
                }
            }
        }
        if let Some(phase) = self.phases.get_mut(phase_id) {
            phase.start();
            self.current_phase = Some(phase_id);
            self.selected_index = phase_id;
            return true;
        }
        false
    }

    pub fn complete_current(&mut self) {
        if let Some(current) = self.current_phase {
            if let Some(phase) = self.phases.get_mut(current) {
                phase.complete();
            }
        }
    }

    pub fn fail_current(&mut self) {
        if let Some(current) = self.current_phase {
            if let Some(phase) = self.phases.get_mut(current) {
                phase.fail();
            }
        }
    }
}
