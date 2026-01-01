use anyhow::Result;
use std::process::Command;

use crate::data::{Action, ActionCategory};

pub fn execute_action(action: &Action) -> Result<()> {
    println!(
        "\x1b[36m>\x1b[0m Executing: \x1b[1;35m{}\x1b[0m",
        action.name
    );
    println!("\x1b[90m  {}\x1b[0m", action.description);
    println!();

    match action.category {
        ActionCategory::Mcp => {
            println!("\x1b[33mMCP Tool:\x1b[0m {}", action.invocation);
            println!("\x1b[90mCopy this to your AI assistant or run via MCP client\x1b[0m");
        }
        ActionCategory::Agent => {
            println!("\x1b[32mAgent:\x1b[0m {}", action.invocation);
            println!("\x1b[90mUse this agent type in your Task tool calls\x1b[0m");
        }
        ActionCategory::Skill => {
            println!("\x1b[34mSkill:\x1b[0m {}", action.invocation);
            println!("\x1b[90mInvoke this skill in your AI assistant\x1b[0m");
        }
        ActionCategory::Command => {
            println!("\x1b[35mRunning command:\x1b[0m {}", action.invocation);
            println!();

            let status = Command::new("sh")
                .arg("-c")
                .arg(&action.invocation)
                .status()?;

            if !status.success() {
                eprintln!("\x1b[31mCommand exited with status: {}\x1b[0m", status);
            }
        }
    }

    Ok(())
}
