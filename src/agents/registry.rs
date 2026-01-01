use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::models::{AgentId, AgentInstance, AgentState};

pub struct HealthCheck {
    pub agent_id: AgentId,
    pub is_healthy: bool,
    pub last_check: Instant,
    pub response_time: Duration,
    pub error_count: u32,
}

impl HealthCheck {
    pub fn healthy(agent_id: AgentId, response_time: Duration) -> Self {
        Self {
            agent_id,
            is_healthy: true,
            last_check: Instant::now(),
            response_time,
            error_count: 0,
        }
    }

    pub fn unhealthy(agent_id: AgentId, error_count: u32) -> Self {
        Self {
            agent_id,
            is_healthy: false,
            last_check: Instant::now(),
            response_time: Duration::ZERO,
            error_count,
        }
    }
}

pub struct AgentRegistry {
    agents: HashMap<AgentId, AgentInstance>,
    health_checks: HashMap<AgentId, HealthCheck>,
    check_interval: Duration,
    unhealthy_threshold: u32,
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            health_checks: HashMap::new(),
            check_interval: Duration::from_secs(5),
            unhealthy_threshold: 3,
        }
    }

    pub fn register(&mut self, agent: AgentInstance) {
        let id = agent.id.clone();
        self.agents.insert(id.clone(), agent);
        self.health_checks.insert(id.clone(), HealthCheck::healthy(id, Duration::ZERO));
    }

    pub fn unregister(&mut self, id: &AgentId) -> Option<AgentInstance> {
        self.health_checks.remove(id);
        self.agents.remove(id)
    }

    pub fn get(&self, id: &AgentId) -> Option<&AgentInstance> {
        self.agents.get(id)
    }

    pub fn get_mut(&mut self, id: &AgentId) -> Option<&mut AgentInstance> {
        self.agents.get_mut(id)
    }

    pub fn list_active(&self) -> Vec<&AgentInstance> {
        self.agents.values().filter(|a| a.state == AgentState::Running).collect()
    }

    pub fn list_unhealthy(&self) -> Vec<&AgentId> {
        self.health_checks.values().filter(|h| !h.is_healthy).map(|h| &h.agent_id).collect()
    }

    pub fn record_health(&mut self, id: &AgentId, is_healthy: bool, response_time: Duration) {
        if let Some(check) = self.health_checks.get_mut(id) {
            check.last_check = Instant::now();
            check.response_time = response_time;
            if is_healthy {
                check.is_healthy = true;
                check.error_count = 0;
            } else {
                check.error_count += 1;
                check.is_healthy = check.error_count < self.unhealthy_threshold;
            }
        }
    }

    pub fn needs_check(&self) -> Vec<AgentId> {
        let now = Instant::now();
        self.health_checks
            .iter()
            .filter(|(id, check)| {
                self.agents.get(*id).map(|a| a.state == AgentState::Running).unwrap_or(false)
                    && now.duration_since(check.last_check) >= self.check_interval
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn healthy_count(&self) -> usize {
        self.health_checks.values().filter(|h| h.is_healthy).count()
    }
}
