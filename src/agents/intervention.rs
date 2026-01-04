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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterventionCommand {
    StopAgent(AgentId),
    StopAll,
    UndoLast,
    InjectPrompt(AgentId),
    ResumeAgent(AgentId),
}

#[derive(Debug, Clone)]
pub struct ConfirmationDialog {
    pub command: InterventionCommand,
    pub title: String,
    pub message: String,
    pub confirm_key: char,
    pub cancel_key: char,
}

impl ConfirmationDialog {
    pub fn for_stop(agent_id: AgentId) -> Self {
        Self {
            command: InterventionCommand::StopAgent(agent_id),
            title: String::from("Stop Agent"),
            message: format!("Stop agent #{}? This cannot be undone.", agent_id.0),
            confirm_key: 'y',
            cancel_key: 'n',
        }
    }

    pub fn for_stop_all() -> Self {
        Self {
            command: InterventionCommand::StopAll,
            title: String::from("Stop All Agents"),
            message: String::from("Stop ALL running agents? This cannot be undone."),
            confirm_key: 'y',
            cancel_key: 'n',
        }
    }

    pub fn for_undo() -> Self {
        Self {
            command: InterventionCommand::UndoLast,
            title: String::from("Undo Last Action"),
            message: String::from("Revert the last checkpoint?"),
            confirm_key: 'y',
            cancel_key: 'n',
        }
    }
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
    pub fn paused_agents(&self) -> &[AgentId] { &self.paused_agents }
    pub fn stop_all(&mut self) { self.paused_agents.clear(); }
    pub fn tick(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_matches_pattern() {
        let rule = InterventionRule::new(
            "dangerous",
            "rm -rf",
            TriggerSeverity::Fatal,
            TriggerAction::Stop,
        );
        assert!(rule.matches("running rm -rf /tmp"));
        assert!(!rule.matches("removing files safely"));
    }

    #[test]
    fn test_disabled_rule_does_not_match() {
        let mut rule = InterventionRule::new(
            "disabled",
            "pattern",
            TriggerSeverity::Warning,
            TriggerAction::Log,
        );
        rule.enabled = false;
        assert!(!rule.matches("text with pattern inside"));
    }

    #[test]
    fn test_monitor_check_triggers_event() {
        let mut monitor = InterventionMonitor::with_default_rules();
        let agent = AgentId(1);
        let event = monitor.check(agent, "executing rm -rf / now");
        assert!(event.is_some());
        let e = event.unwrap();
        assert_eq!(e.rule_name, "rm_rf");
        assert_eq!(e.severity, TriggerSeverity::Fatal);
        assert!(monitor.is_paused(&AgentId(1)));
    }

    #[test]
    fn test_monitor_resume_agent() {
        let mut monitor = InterventionMonitor::new();
        let agent = AgentId(42);
        monitor.check(agent, "sudo rm important");
        monitor.add_rule(InterventionRule::new(
            "sudo",
            "sudo",
            TriggerSeverity::Critical,
            TriggerAction::Pause,
        ));
        monitor.check(agent, "sudo rm important");
        assert!(monitor.is_paused(&agent));
        monitor.resume(&agent);
        assert!(!monitor.is_paused(&agent));
    }

    #[test]
    fn test_confirmation_dialog_for_stop() {
        let dialog = ConfirmationDialog::for_stop(AgentId(5));
        assert!(matches!(dialog.command, InterventionCommand::StopAgent(AgentId(5))));
        assert_eq!(dialog.confirm_key, 'y');
        assert_eq!(dialog.cancel_key, 'n');
    }
}
