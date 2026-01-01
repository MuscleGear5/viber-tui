//! ActionRegistry for loading and querying actions
//!
//! Provides methods to load actions from YAML files and query them.

use anyhow::{Context, Result};
use std::path::Path;

use super::models::{Action, ActionCategory, ActionsFile};

/// Registry holding all loaded actions
#[derive(Debug, Clone, Default)]
pub struct ActionRegistry {
    actions: Vec<Action>,
}

impl ActionRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Load actions from a YAML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read actions file: {}", path.display()))?;
        Self::load_from_str(&content)
    }

    /// Load actions from a YAML string
    pub fn load_from_str(yaml: &str) -> Result<Self> {
        let file: ActionsFile =
            serde_yaml::from_str(yaml).context("Failed to parse actions YAML")?;
        Ok(Self {
            actions: file.actions,
        })
    }

    /// Load actions from embedded default if file doesn't exist
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            Self::load_from_file(path)
        } else {
            // Return empty registry if no file
            Ok(Self::new())
        }
    }

    /// Get all actions
    pub fn all(&self) -> &[Action] {
        &self.actions
    }

    /// Get actions by category
    pub fn by_category(&self, category: ActionCategory) -> Vec<&Action> {
        self.actions
            .iter()
            .filter(|a| a.category == category)
            .collect()
    }

    /// Get action by ID
    pub fn get(&self, id: &str) -> Option<&Action> {
        self.actions.iter().find(|a| a.id == id)
    }

    /// Get total count
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Get iterator over actions
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_action() {
        let yaml = r#"
actions:
  - id: "mcp.test"
    name: "Test MCP"
    category: mcp
    description: "A test MCP tool"
    invocation: "ncp_code test()"
    keywords:
      - test
      - example
"#;
        let registry = ActionRegistry::load_from_str(yaml).unwrap();
        assert_eq!(registry.len(), 1);
        let action = registry.get("mcp.test").unwrap();
        assert_eq!(action.name, "Test MCP");
        assert_eq!(action.category, ActionCategory::Mcp);
    }

    #[test]
    fn test_category_icon() {
        assert_eq!(ActionCategory::Mcp.icon(), "\u{f0e7}");
        assert_eq!(ActionCategory::Agent.icon(), "\u{f544}");
        assert_eq!(ActionCategory::Skill.icon(), "\u{e370}");
        assert_eq!(ActionCategory::Command.icon(), "\u{f120}");
    }
}
