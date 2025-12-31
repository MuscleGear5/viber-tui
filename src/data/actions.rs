//! Action data model for VIBER TUI
//!
//! Defines the Action struct and registry for loading/searching actions.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Category of action - determines color theming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionCategory {
    Mcp,
    Agent,
    Skill,
    Command,
}

impl ActionCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionCategory::Mcp => "MCP",
            ActionCategory::Agent => "Agent",
            ActionCategory::Skill => "Skill",
            ActionCategory::Command => "Command",
        }
    }

    pub fn icon(&self) -> char {
        match self {
            ActionCategory::Mcp => '⚡',    // Lightning for MCP tools
            ActionCategory::Agent => '🤖',  // Robot for agents
            ActionCategory::Skill => '✨',  // Sparkles for skills
            ActionCategory::Command => '/', // Slash for commands
        }
    }
}

/// Parameter type for action invocation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamType {
    String,
    Number,
    Boolean,
    Path,
    Choice(Vec<String>),
}

impl Default for ParamType {
    fn default() -> Self {
        ParamType::String
    }
}

/// A parameter for an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(default)]
    pub param_type: ParamType,
    #[serde(default)]
    pub required: bool,
    pub default: Option<String>,
    pub description: Option<String>,
}

/// An action that can be invoked from the launcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Unique identifier (e.g., "mcp.ncp-coding", "agent.explore")
    pub id: String,
    /// Display name
    pub name: String,
    /// Category for theming
    pub category: ActionCategory,
    /// Subcategory for grouping (e.g., "research", "coding", "productivity")
    #[serde(default)]
    pub subcategory: Option<String>,
    /// Short description
    pub description: String,
    /// Search keywords
    #[serde(default)]
    pub keywords: Vec<String>,
    /// How to invoke this action (template string)
    pub invocation: String,
    /// Parameters for the action
    #[serde(default)]
    pub params: Vec<Param>,
    /// Custom icon override (single char)
    #[serde(default)]
    pub icon: Option<char>,
}

impl Action {
    /// Get the display icon for this action
    pub fn display_icon(&self) -> char {
        self.icon.unwrap_or_else(|| self.category.icon())
    }

    /// Get all searchable text combined
    pub fn searchable_text(&self) -> String {
        let mut text = format!("{} {} {}", self.name, self.description, self.id);
        if let Some(ref sub) = self.subcategory {
            text.push(' ');
            text.push_str(sub);
        }
        for kw in &self.keywords {
            text.push(' ');
            text.push_str(kw);
        }
        text
    }
}

/// Container for all actions loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsFile {
    pub actions: Vec<Action>,
}

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
            // Return empty registry if no file - actions.yaml will be created separately
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
        assert_eq!(ActionCategory::Mcp.icon(), '⚡');
        assert_eq!(ActionCategory::Agent.icon(), '🤖');
        assert_eq!(ActionCategory::Skill.icon(), '✨');
        assert_eq!(ActionCategory::Command.icon(), '/');
    }
}
