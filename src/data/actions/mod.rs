//! Action data model for VIBER TUI
//!
//! Defines the Action struct and registry for loading/searching actions.

mod models;
mod registry;

pub use models::{Action, ActionCategory};
pub use registry::ActionRegistry;
