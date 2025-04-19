use colored::Colorize;
use std::{io::Write, process::Command, process::Stdio};

/// runs sudo commands
pub fn run_sudo_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "failed to execute command:".red(), command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
} // run_sudo_command

/// runs user commands
pub fn run_user_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "failed to execute command:".red(), command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
} // run_user_command

/// runs sudo commands with stdin
pub fn run_sudo_command_with_stdin(
    command: &str,
    args: &[&str],
    stdin_content: String,
) -> Result<(), String> {
    let mut cmd = Command::new("sudo")
        .arg(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to spawn command `{}`: {}", command, e))?;

    if let Some(mut stdin) = cmd.stdin.take() {
        stdin
            .write_all(stdin_content.as_bytes())
            .map_err(|e| format!("failed to write to stdin of `{}`: {}", command, e))?;
    }

    let output = cmd
        .wait_with_output()
        .map_err(|e| format!("failed to wait for command `{}`: {}", command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command `{}` failed:\nStdout: {}\nStderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
} // run_sudo_command_with_stdin
