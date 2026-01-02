use std::time::Instant;

use super::AgentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerSeverity {
    Warning,
    Critical,
    Fatal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerAction {
    Log,
    Notify,
    Pause,
    Stop,
}

#[derive(Debug, Clone)]
pub struct InterventionRule {
    pub name: String,
    pub pattern: String,
    pub severity: TriggerSeverity,
    pub action: TriggerAction,
    pub enabled: bool,
}

impl InterventionRule {
    pub fn new(name: impl Into<String>, pattern: impl Into<String>, severity: TriggerSeverity, action: TriggerAction) -> Self {
        Self { name: name.into(), pattern: pattern.into(), severity, action, enabled: true }
    }

    pub fn matches(&self, text: &str) -> bool {
        self.enabled && text.contains(&self.pattern)
    }
}

#[derive(Debug, Clone)]
pub struct TriggerEvent {
    pub rule_name: String,
    pub agent_id: AgentId,
    pub matched_text: String,
    pub severity: TriggerSeverity,
    pub action: TriggerAction,
    pub timestamp: Instant,
}

#[derive(Debug, Default)]
pub struct InterventionMonitor {
    rules: Vec<InterventionRule>,
    events: Vec<TriggerEvent>,
    paused_agents: Vec<AgentId>,
}

impl InterventionMonitor {
    pub fn new() -> Self { Self::default() }

    pub fn with_default_rules() -> Self {
        let mut monitor = Self::new();
        monitor.add_rule(InterventionRule::new("rm_rf", "rm -rf /", TriggerSeverity::Fatal, TriggerAction::Stop));
        monitor.add_rule(InterventionRule::new("sudo_rm", "sudo rm", TriggerSeverity::Critical, TriggerAction::Pause));
        monitor.add_rule(InterventionRule::new("force_push", "git push --force", TriggerSeverity::Critical, TriggerAction::Pause));
        monitor.add_rule(InterventionRule::new("drop_table", "DROP TABLE", TriggerSeverity::Critical, TriggerAction::Pause));
        monitor.add_rule(InterventionRule::new("truncate", "TRUNCATE", TriggerSeverity::Warning, TriggerAction::Notify));
        monitor
    }

    pub fn add_rule(&mut self, rule: InterventionRule) { self.rules.push(rule); }

    pub fn check(&mut self, agent_id: AgentId, text: &str) -> Option<TriggerEvent> {
        for rule in &self.rules {
            if rule.matches(text) {
                let event = TriggerEvent {
                    rule_name: rule.name.clone(),
                    agent_id: agent_id.clone(),
                    matched_text: text.chars().take(100).collect(),
                    severity: rule.severity,
                    action: rule.action,
                    timestamp: Instant::now(),
                };
                if rule.action == TriggerAction::Pause || rule.action == TriggerAction::Stop {
                    self.paused_agents.push(agent_id.clone());
                }
                self.events.push(event.clone());
                return Some(event);
            }
        }
        None
    }

    pub fn is_paused(&self, agent_id: &AgentId) -> bool { self.paused_agents.contains(agent_id) }
    pub fn resume(&mut self, agent_id: &AgentId) { self.paused_agents.retain(|id| id != agent_id); }
    pub fn events(&self) -> &[TriggerEvent] { &self.events }
    pub fn clear_events(&mut self) { self.events.clear(); }
    pub fn tick(&mut self) {}
}
