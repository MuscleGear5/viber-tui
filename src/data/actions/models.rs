//! Action data models for VIBER TUI
//!
//! Contains the core types: ActionCategory, ParamType, Param, Action, ActionsFile

use serde::{Deserialize, Serialize};

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

    pub fn icon(&self) -> &'static str {
        match self {
            ActionCategory::Mcp => "\u{f0e7}",
            ActionCategory::Agent => "\u{f544}",
            ActionCategory::Skill => "\u{e370}",
            ActionCategory::Command => "\u{f120}",
        }
    }
}

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
    pub icon: Option<String>,
}

impl Action {
    pub fn display_icon(&self) -> &str {
        self.icon.as_deref().unwrap_or_else(|| self.category.icon())
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
