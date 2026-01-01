use serde::{Deserialize, Serialize};
use std::path::Path;

use super::models::{Phase, PhaseStatus};
use super::state::WorkflowState;

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub phases: Vec<PhaseData>,
    pub current_phase: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct PhaseData {
    pub id: usize,
    pub name: String,
    pub status: String,
    pub progress: u8,
    pub dependencies: Vec<usize>,
}

impl From<&Phase> for PhaseData {
    fn from(p: &Phase) -> Self {
        Self {
            id: p.id,
            name: p.name.clone(),
            status: match p.status {
                PhaseStatus::Pending => "pending",
                PhaseStatus::InProgress => "in_progress",
                PhaseStatus::Completed => "completed",
                PhaseStatus::Failed => "failed",
                PhaseStatus::Skipped => "skipped",
            }
            .into(),
            progress: p.progress,
            dependencies: p.dependencies.clone(),
        }
    }
}

impl SessionData {
    pub fn from_state(state: &WorkflowState) -> Self {
        Self {
            phases: state.phases.iter().map(PhaseData::from).collect(),
            current_phase: state.current_phase,
        }
    }

    pub fn apply_to(&self, state: &mut WorkflowState) {
        for pd in &self.phases {
            if let Some(phase) = state.phases.get_mut(pd.id) {
                phase.status = match pd.status.as_str() {
                    "in_progress" => PhaseStatus::InProgress,
                    "completed" => PhaseStatus::Completed,
                    "failed" => PhaseStatus::Failed,
                    "skipped" => PhaseStatus::Skipped,
                    _ => PhaseStatus::Pending,
                };
                phase.progress = pd.progress;
            }
        }
        state.current_phase = self.current_phase;
        if let Some(idx) = self.current_phase {
            state.selected_index = idx;
        }
    }
}

pub fn save_session(state: &WorkflowState, path: &Path) -> std::io::Result<()> {
    let data = SessionData::from_state(state);
    let json = serde_json::to_string_pretty(&data)?;
    std::fs::write(path, json)
}

pub fn load_session(path: &Path) -> std::io::Result<SessionData> {
    let json = std::fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
