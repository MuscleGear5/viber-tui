use crate::integrations::mcp::{McpNotification, McpRequest, McpResponse, ToolResult, init_request};
use serde::Deserialize;
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub enum NvimMcpCommand {
    GetTargets,
    Connect { target: String },
    ListBuffers,
    CursorPosition,
    Read { path: String },
    Navigate { path: String, line: u32, character: u32 },
    LspClients,
    LspDiagnostics { buffer_id: u32 },
    ExecLua { code: String },
    Shutdown,
}

pub enum NvimMcpResponse {
    Targets(Vec<NvimTarget>),
    Connected { connection_id: String },
    Buffers(Vec<BufferInfo>),
    Cursor { line: u32, column: u32, buffer_id: u32 },
    FileContent { content: String },
    LspClients(Vec<LspClientInfo>),
    Diagnostics(Vec<DiagnosticInfo>),
    LuaResult { output: String },
    Error { message: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct NvimTarget {
    pub socket_path: String,
    pub connection_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BufferInfo {
    pub id: u32,
    pub name: String,
    pub line_count: u32,
    pub modified: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LspClientInfo {
    pub name: String,
    pub root_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagnosticInfo {
    pub line: u32,
    pub column: u32,
    pub severity: u8,
    pub message: String,
    pub source: Option<String>,
}

pub struct NvimMcpRunner {
    cmd_tx: Sender<NvimMcpCommand>,
    resp_rx: Receiver<NvimMcpResponse>,
    connection_id: Option<String>,
}

impl NvimMcpRunner {
    pub fn spawn() -> anyhow::Result<Self> {
        let (cmd_tx, cmd_rx) = mpsc::channel::<NvimMcpCommand>();
        let (resp_tx, resp_rx) = mpsc::channel::<NvimMcpResponse>();

        thread::spawn(move || {
            if let Err(e) = run_mcp_loop(cmd_rx, resp_tx) {
                eprintln!("MCP runner error: {}", e);
            }
        });

        Ok(Self { cmd_tx, resp_rx, connection_id: None })
    }

    pub fn send(&self, cmd: NvimMcpCommand) -> anyhow::Result<()> {
        self.cmd_tx.send(cmd)?;
        Ok(())
    }

    pub fn try_recv(&mut self) -> Option<NvimMcpResponse> {
        match self.resp_rx.try_recv() {
            Ok(resp) => {
                if let NvimMcpResponse::Connected { ref connection_id } = resp {
                    self.connection_id = Some(connection_id.clone());
                }
                Some(resp)
            }
            Err(_) => None,
        }
    }

    pub fn connection_id(&self) -> Option<&str> {
        self.connection_id.as_deref()
    }

    pub fn shutdown(&self) {
        let _ = self.cmd_tx.send(NvimMcpCommand::Shutdown);
    }
}

fn run_mcp_loop(
    cmd_rx: Receiver<NvimMcpCommand>,
    resp_tx: Sender<NvimMcpResponse>,
) -> anyhow::Result<()> {
    let mut process: Option<McpProcess> = None;
    let mut connection_id: Option<String> = None;

    loop {
        let cmd = match cmd_rx.recv() {
            Ok(c) => c,
            Err(_) => break,
        };

        match cmd {
            NvimMcpCommand::Shutdown => break,
            NvimMcpCommand::GetTargets => {
                let proc = ensure_process(&mut process)?;
                match call_tool(proc, "nvim_get_targets", serde_json::json!({})) {
                    Ok(result) => {
                        let targets = parse_targets(&result);
                        let _ = resp_tx.send(NvimMcpResponse::Targets(targets));
                    }
                    Err(e) => send_error(&resp_tx, e),
                }
            }
            NvimMcpCommand::Connect { target } => {
                let proc = ensure_process(&mut process)?;
                match call_tool(proc, "nvim_connect", serde_json::json!({ "target": target })) {
                    Ok(result) => {
                        if let Some(id) = parse_connection_id(&result) {
                            connection_id = Some(id.clone());
                            let _ = resp_tx.send(NvimMcpResponse::Connected { connection_id: id });
                        }
                    }
                    Err(e) => send_error(&resp_tx, e),
                }
            }
            NvimMcpCommand::ListBuffers => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    match call_tool(proc, "nvim_list_buffers", serde_json::json!({ "connection_id": conn_id })) {
                        Ok(result) => {
                            let buffers = parse_buffers(&result);
                            let _ = resp_tx.send(NvimMcpResponse::Buffers(buffers));
                        }
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
            NvimMcpCommand::CursorPosition => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    match call_tool(proc, "nvim_cursor_position", serde_json::json!({ "connection_id": conn_id })) {
                        Ok(result) => {
                            if let Some((line, col, buf_id)) = parse_cursor(&result) {
                                let _ = resp_tx.send(NvimMcpResponse::Cursor { line, column: col, buffer_id: buf_id });
                            }
                        }
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
            NvimMcpCommand::Read { path } => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    let args = serde_json::json!({
                        "connection_id": conn_id,
                        "document": { "project_relative_path": path }
                    });
                    match call_tool(proc, "nvim_read", args) {
                        Ok(result) => {
                            let content = result.text().unwrap_or("").to_string();
                            let _ = resp_tx.send(NvimMcpResponse::FileContent { content });
                        }
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
            NvimMcpCommand::Navigate { path, line, character } => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    let args = serde_json::json!({
                        "connection_id": conn_id,
                        "document": { "project_relative_path": path },
                        "line": line,
                        "character": character
                    });
                    match call_tool(proc, "nvim_navigate", args) {
                        Ok(_) => {}
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
            NvimMcpCommand::LspClients => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    match call_tool(proc, "nvim_lsp_clients", serde_json::json!({ "connection_id": conn_id })) {
                        Ok(result) => {
                            let clients = parse_lsp_clients(&result);
                            let _ = resp_tx.send(NvimMcpResponse::LspClients(clients));
                        }
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
            NvimMcpCommand::LspDiagnostics { buffer_id } => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    let args = serde_json::json!({
                        "connection_id": conn_id,
                        "id": buffer_id
                    });
                    match call_tool(proc, "nvim_buffer_diagnostics", args) {
                        Ok(result) => {
                            let diags = parse_diagnostics(&result);
                            let _ = resp_tx.send(NvimMcpResponse::Diagnostics(diags));
                        }
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
            NvimMcpCommand::ExecLua { code } => {
                if let (Some(proc), Some(ref conn_id)) = (process.as_mut(), &connection_id) {
                    let args = serde_json::json!({
                        "connection_id": conn_id,
                        "code": code
                    });
                    match call_tool(proc, "nvim_exec_lua", args) {
                        Ok(result) => {
                            let output = result.text().unwrap_or("").to_string();
                            let _ = resp_tx.send(NvimMcpResponse::LuaResult { output });
                        }
                        Err(e) => send_error(&resp_tx, e),
                    }
                }
            }
        }
    }
    Ok(())
}

struct McpProcess {
    child: Child,
}

fn ensure_process(process: &mut Option<McpProcess>) -> anyhow::Result<&mut McpProcess> {
    if process.is_none() {
        *process = Some(spawn_nvim_mcp()?);
    }
    Ok(process.as_mut().expect("process should exist"))
}

fn spawn_nvim_mcp() -> anyhow::Result<McpProcess> {
    let mut child = Command::new("npx")
        .args(["@anthropics/nvim-mcp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let stdin = child.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("No stdin"))?;
    let stdout = child.stdout.as_mut().ok_or_else(|| anyhow::anyhow!("No stdout"))?;

    let init = init_request();
    writeln!(stdin, "{}", serde_json::to_string(&init)?)?;
    stdin.flush()?;

    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let _: McpResponse = serde_json::from_str(&line)?;

    let notif = McpNotification::initialized();
    let stdin = child.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("No stdin"))?;
    writeln!(stdin, "{}", serde_json::to_string(&notif)?)?;
    stdin.flush()?;

    Ok(McpProcess { child })
}

fn call_tool(proc: &mut McpProcess, tool: &str, args: Value) -> anyhow::Result<ToolResult> {
    let stdin = proc.child.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("No stdin"))?;
    let req = McpRequest::call_tool(tool, args);
    writeln!(stdin, "{}", serde_json::to_string(&req)?)?;
    stdin.flush()?;

    let stdout = proc.child.stdout.as_mut().ok_or_else(|| anyhow::anyhow!("No stdout"))?;
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let resp: McpResponse = serde_json::from_str(&line)?;
    let result = resp.into_result()?;
    let tool_result: ToolResult = serde_json::from_value(result)?;
    Ok(tool_result)
}

fn send_error(tx: &Sender<NvimMcpResponse>, e: impl std::fmt::Display) {
    let _ = tx.send(NvimMcpResponse::Error { message: e.to_string() });
}

fn parse_targets(result: &ToolResult) -> Vec<NvimTarget> {
    result.text().and_then(|t| serde_json::from_str(t).ok()).unwrap_or_default()
}

fn parse_connection_id(result: &ToolResult) -> Option<String> {
    result.text().and_then(|t| {
        serde_json::from_str::<Value>(t).ok()?.get("connection_id")?.as_str().map(String::from)
    })
}

fn parse_buffers(result: &ToolResult) -> Vec<BufferInfo> {
    result.text().and_then(|t| serde_json::from_str(t).ok()).unwrap_or_default()
}

fn parse_cursor(result: &ToolResult) -> Option<(u32, u32, u32)> {
    result.text().and_then(|t| {
        let v: Value = serde_json::from_str(t).ok()?;
        let line = v.get("line")?.as_u64()? as u32;
        let col = v.get("character")?.as_u64()? as u32;
        let buf = v.get("buffer_id").and_then(|b| b.as_u64()).unwrap_or(0) as u32;
        Some((line, col, buf))
    })
}

fn parse_lsp_clients(result: &ToolResult) -> Vec<LspClientInfo> {
    result.text().and_then(|t| serde_json::from_str(t).ok()).unwrap_or_default()
}

fn parse_diagnostics(result: &ToolResult) -> Vec<DiagnosticInfo> {
    result.text().and_then(|t| serde_json::from_str(t).ok()).unwrap_or_default()
}
