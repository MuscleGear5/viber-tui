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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = ViberState::new();
        assert_eq!(state.phase, ViberPhase::Idle);
        assert_eq!(state.vibe_level, VibeLevel::Nominal);
        assert_eq!(state.active_agents, 0);
        assert!(!state.is_intervening);
    }

    #[test]
    fn test_builder_pattern() {
        let state = ViberState::new()
            .with_phase(ViberPhase::Implementation)
            .with_agents(3, 5)
            .with_vibe(VibeLevel::Warning, 0.75);

        assert_eq!(state.phase, ViberPhase::Implementation);
        assert_eq!(state.active_agents, 3);
        assert_eq!(state.total_agents, 5);
        assert_eq!(state.vibe_level, VibeLevel::Warning);
        assert_eq!(state.compliance_percent(), 75);
    }

    #[test]
    fn test_compliance_clamping() {
        let over = ViberState::new().with_vibe(VibeLevel::Nominal, 1.5);
        assert_eq!(over.spec_compliance, 1.0);

        let under = ViberState::new().with_vibe(VibeLevel::Nominal, -0.5);
        assert_eq!(under.spec_compliance, 0.0);
    }

    #[test]
    fn test_intervention_lifecycle() {
        let mut state = ViberState::new();
        assert!(!state.is_intervening);
        assert!(state.active_power.is_none());

        state.set_intervening(ViberPower::Stop);
        assert!(state.is_intervening);
        assert_eq!(state.active_power, Some(ViberPower::Stop));

        state.clear_intervention();
        assert!(!state.is_intervening);
        assert!(state.active_power.is_none());
    }
}
