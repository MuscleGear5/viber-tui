use super::models::{ViberPhase, ViberPower, VibeLevel};

#[derive(Debug, Clone, Default)]
pub struct ViberState {
    pub phase: ViberPhase,
    pub vibe_level: VibeLevel,
    pub active_agents: u8,
    pub total_agents: u8,
    pub active_power: Option<ViberPower>,
    pub is_intervening: bool,
    pub spec_compliance: f32,
}

impl ViberState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_phase(mut self, phase: ViberPhase) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_agents(mut self, active: u8, total: u8) -> Self {
        self.active_agents = active;
        self.total_agents = total;
        self
    }

    pub fn with_vibe(mut self, level: VibeLevel, compliance: f32) -> Self {
        self.vibe_level = level;
        self.spec_compliance = compliance.clamp(0.0, 1.0);
        self
    }

    pub fn set_intervening(&mut self, power: ViberPower) {
        self.is_intervening = true;
        self.active_power = Some(power);
    }

    pub fn clear_intervention(&mut self) {
        self.is_intervening = false;
        self.active_power = None;
    }

    pub fn compliance_percent(&self) -> u8 {
        (self.spec_compliance * 100.0) as u8
    }
}
