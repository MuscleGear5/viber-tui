mod dispatch;
mod intervention;
mod models;
mod pool;
mod protocol;
mod registry;
mod undo;

pub use dispatch::{AgentCommand, AgentEvent, AgentStatusSummary, DispatchResult};
pub use intervention::{
    ConfirmationDialog, InterventionCommand, InterventionMonitor,
    TriggerAction, TriggerEvent, TriggerSeverity,
};
pub use models::AgentId;
pub use protocol::AgentController;
pub use pool::{AgentPool, AgentTask};
pub use registry::AgentRegistry;
pub use undo::UndoStack;
