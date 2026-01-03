//! MCP (Model Context Protocol) JSON-RPC types and messaging
//!
//! Provides the low-level protocol types for communicating with MCP servers
//! like nvim-mcp. Uses JSON-RPC 2.0 over stdio.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};

static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a unique request ID
pub fn next_request_id() -> u64 {
    REQUEST_ID.fetch_add(1, Ordering::SeqCst)
}

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize)]
pub struct McpRequest {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl McpRequest {
    pub fn new(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0",
            id: next_request_id(),
            method: method.into(),
            params,
        }
    }

    /// Create a tools/call request for invoking MCP tools
    pub fn call_tool(tool_name: &str, arguments: Value) -> Self {
        Self::new(
            "tools/call",
            Some(serde_json::json!({
                "name": tool_name,
                "arguments": arguments
            })),
        )
    }
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<McpError>,
}

impl McpResponse {
    pub fn is_success(&self) -> bool {
        self.error.is_none() && self.result.is_some()
    }

    pub fn into_result(self) -> Result<Value, McpError> {
        if let Some(err) = self.error {
            Err(err)
        } else {
            Ok(self.result.unwrap_or(Value::Null))
        }
    }
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub data: Option<Value>,
}

impl std::fmt::Display for McpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MCP error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for McpError {}

/// Tool call result content
#[derive(Debug, Clone, Deserialize)]
pub struct ToolResult {
    #[serde(default)]
    pub content: Vec<ToolContent>,
    #[serde(default)]
    pub is_error: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ToolContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(other)]
    Unknown,
}

impl ToolResult {
    /// Extract text content from the result
    pub fn text(&self) -> Option<&str> {
        self.content.iter().find_map(|c| match c {
            ToolContent::Text { text } => Some(text.as_str()),
            _ => None,
        })
    }
}

/// MCP initialization request
pub fn init_request() -> McpRequest {
    McpRequest::new(
        "initialize",
        Some(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "viber-tui",
                "version": "0.1.0"
            }
        })),
    )
}

/// MCP initialized notification (no response expected)
#[derive(Debug, Clone, Serialize)]
pub struct McpNotification {
    pub jsonrpc: &'static str,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl McpNotification {
    pub fn initialized() -> Self {
        Self {
            jsonrpc: "2.0",
            method: "notifications/initialized".to_string(),
            params: None,
        }
    }
}
