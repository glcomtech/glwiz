/*
 *  gnulinwiz AKA GNU/Linux Config Wizard: The ultimate post-installation setup assistant for Linux,
 *  streamlining your configuration process with ease and precision.
 *
 *  Copyright (C) 2025  Andrew Kushyk
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use colored::Colorize;
use std::{io::Write, process::Command, process::Stdio};

/// Executes a command with `sudo` privileges and captures its output.
///
/// # Arguments
/// * `command` - The command to execute (e.g., "pacman", "systemctl").
/// * `args` - A slice of arguments to pass to the command.
///
/// # Returns
/// * `Ok(())` if the command executes successfully.
/// * `Err(String)` with a formatted error message if the command fails or cannot be executed.
///
/// # Examples
/// ```
/// let result = run_sudo_command("pacman", &["update"]);
/// assert!(result.is_ok());
/// ```
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
}

/// Executes a command as the current user and captures its output.
///
/// # Arguments
/// * `command` - The command to execute (e.g., "ls", "git").
/// * `args` - A slice of arguments to pass to the command.
///
/// # Returns
/// * `Ok(())` if the command executes successfully.
/// * `Err(String)` with a formatted error message if the command fails or cannot be executed.
///
/// # Examples
/// ```
/// let result = run_user_command("ls", &["-l"]);
/// assert!(result.is_ok());
/// ```
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
}

/// Executes a command with `sudo` privileges, passing input via stdin, and captures its output.
///
/// This function is useful for commands that require input, such as configuration tools or scripts
/// that read from stdin.
///
/// # Arguments
/// * `command` - The command to execute (e.g., "tee", "passwd").
/// * `args` - A slice of arguments to pass to the command.
/// * `stdin_content` - The content to write to the command's stdin.
///
/// # Returns
/// * `Ok(())` if the command executes successfully.
/// * `Err(String)` with a formatted error message if the command fails, cannot be spawned,
///   or if writing to stdin fails.
///
/// # Examples
/// ```
/// let content = "some config data".to_string();
/// let result = run_sudo_command_with_stdin("tee", &["/etc/somefile"], content);
/// assert!(result.is_ok());
/// ```
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
}
