use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueType {
    Task,
    Bug,
    Feature,
    Epic,
}

impl IssueType {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Task => "task", Self::Bug => "bug", Self::Feature => "feature", Self::Epic => "epic" }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueStatus {
    Open,
    InProgress,
    Closed,
    Blocked,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Open => "open", Self::InProgress => "in_progress", Self::Closed => "closed", Self::Blocked => "blocked" }
    }
    pub fn icon(&self) -> &'static str {
        match self { Self::Open => "\u{F0130}", Self::InProgress => "\u{F110A}", Self::Closed => "\u{F05E0}", Self::Blocked => "\u{F073A}" }
    }
}

#[derive(Debug, Clone)]
pub struct BeadsIssue {
    pub id: String,
    pub title: String,
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: u8,
    pub assignee: Option<String>,
    pub blocked_by: Vec<String>,
    pub blocks: Vec<String>,
}

impl BeadsIssue {
    pub fn priority_icon(&self) -> &'static str {
        match self.priority { 0 => "\u{F0F23}", 1 => "\u{F005D}", 2 => "\u{F0060}", 3 => "\u{F0062}", _ => "\u{F0063}" }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeadsCommand {
    List,
    Ready,
    Show,
    Create,
    Update,
    Close,
    Blocked,
    Stats,
    Sync,
}

impl BeadsCommand {
    pub fn as_str(&self) -> &'static str {
        match self { Self::List => "list", Self::Ready => "ready", Self::Show => "show", Self::Create => "create", Self::Update => "update", Self::Close => "close", Self::Blocked => "blocked", Self::Stats => "stats", Self::Sync => "sync" }
    }
}

pub struct BeadsClient {
    pub project_path: Option<String>,
}

impl BeadsClient {
    pub fn new() -> Self {
        Self { project_path: None }
    }

    pub fn with_project(path: String) -> Self {
        Self { project_path: Some(path) }
    }

    fn run_bd(&self, args: &[&str]) -> Result<String, String> {
        let mut cmd = Command::new("bd");
        cmd.args(args).arg("--json");
        if let Some(ref path) = self.project_path {
            cmd.current_dir(path);
        }
        cmd.output()
            .map_err(|e| format!("Failed to run bd: {}", e))
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout).map_err(|e| e.to_string())
                } else {
                    Err(String::from_utf8_lossy(&o.stderr).to_string())
                }
            })
    }

    pub fn ready(&self) -> Result<String, String> {
        self.run_bd(&["ready"])
    }

    pub fn list(&self, status: Option<IssueStatus>) -> Result<String, String> {
        match status {
            Some(s) => self.run_bd(&["list", "--status", s.as_str()]),
            None => self.run_bd(&["list"]),
        }
    }

    pub fn show(&self, id: &str) -> Result<String, String> {
        self.run_bd(&["show", id])
    }

    pub fn update_status(&self, id: &str, status: IssueStatus) -> Result<String, String> {
        self.run_bd(&["update", id, "--status", status.as_str()])
    }

    pub fn close(&self, id: &str, reason: Option<&str>) -> Result<String, String> {
        match reason {
            Some(r) => self.run_bd(&["close", id, "--reason", r]),
            None => self.run_bd(&["close", id]),
        }
    }

    pub fn stats(&self) -> Result<String, String> {
        self.run_bd(&["stats"])
    }
}

impl Default for BeadsClient {
    fn default() -> Self {
        Self::new()
    }
}
