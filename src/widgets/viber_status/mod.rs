//! VIBER status panel widget - shows God Agent state with animated eye.

mod models;
mod render;
mod state;
mod widget;

pub use models::{ViberPhase, ViberPower, VibeLevel};
pub use state::ViberState;
pub use widget::ViberStatusPanel;
