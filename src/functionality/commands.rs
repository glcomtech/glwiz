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
use std::{
    io::Write,
    process::{Command, Stdio},
};

/// Executes a system command with `sudo` privileges, requiring elevated permissions.
///
/// This function runs a specified command with `sudo`, passing the provided arguments, and captures
/// its output. It is used in the "gnulinwiz" project for tasks requiring root access, such as copying
/// configuration files to system directories or applying iptables rules. The function ensures robust
/// error handling by reporting command failures with detailed stdout and stderr messages.
///
/// # Arguments
/// * `command` - The command to execute (e.g., `"cp"`, `"iptables-restore"`).
/// * `args` - A slice of arguments to pass to the command (e.g., `&["-r", "/src", "/dest"]`).
///
/// # Returns
/// * `Ok(())` - The command executed successfully.
/// * `Err(String)` - An error message describing the failure, including stdout and stderr if applicable.
///
/// # Errors
/// Returns an error if:
/// - The `sudo` command fails to spawn (e.g., `sudo` is not installed).
/// - The command exits with a non-zero status, indicating failure.
/// - The command’s output cannot be captured or processed.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::commands::run_sudo_command;
/// let result = run_sudo_command("cp", &["/src/file", "/dest/file"]);
/// match result {
///     Ok(()) => println!("File copied successfully"),
///     Err(e) => eprintln!("Failed to copy file: {}", e),
/// }
/// ```
///
/// # Safety
/// Use caution when running commands with `sudo`, as they execute with root privileges and can modify
/// critical system files. Ensure `command` and `args` are validated to prevent unintended consequences.
pub fn run_sudo_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "Failed to execute:".red(), command, e))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Executes a system command as the current user, without elevated privileges.
///
/// This function runs a specified command with the provided arguments as the current user, capturing
/// its output. It is used in the "gnulinwiz" project for tasks that do not require root access, such as
/// cloning Git repositories for Zsh plugins. The function provides detailed error messages for failed
/// commands, including stdout and stderr.
///
/// # Arguments
/// * `command` - The command to execute (e.g., `"git"`, `"bash"`).
/// * `args` - A slice of arguments to pass to the command (e.g., `&["clone", "url"]`).
///
/// # Returns
/// * `Ok(())` - The command executed successfully.
/// * `Err(String)` - An error message describing the failure, including stdout and stderr if applicable.
///
/// # Errors
/// Returns an error if:
/// - The command fails to spawn (e.g., the command is not installed).
/// - The command exits with a non-zero status, indicating failure.
/// - The command’s output cannot be captured or processed.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::commands::run_user_command;
/// let result = run_user_command("git", &["clone", "https://github.com/repo.git"]);
/// match result {
///     Ok(()) => println!("Repository cloned successfully"),
///     Err(e) => eprintln!("Failed to clone repository: {}", e),
/// }
/// ```
pub fn run_user_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "Failed to execute:".red(), command, e))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Executes a system command with `sudo` privileges, passing input via stdin.
///
/// This function runs a specified command with `sudo`, providing input through stdin and suppressing
/// stdout. It is used in the "gnulinwiz" project for tasks like writing configuration files to system
/// directories using `tee`. The function ensures robust error handling by reporting failures with
/// detailed stdout and stderr messages.
///
/// # Arguments
/// * `command` - The command to execute (e.g., `"tee"`, `"bash"`).
/// * `args` - A slice of arguments to pass to the command (e.g., `&["/etc/file"]`).
/// * `stdin_content` - The input to pass to the command’s stdin (e.g., configuration file content).
///
/// # Returns
/// * `Ok(())` - The command executed successfully.
/// * `Err(String)` - An error message describing the failure, including stdout and stderr if applicable.
///
/// # Errors
/// Returns an error if:
/// - The `sudo` command fails to spawn (e.g., `sudo` is not installed).
/// - Writing to the command’s stdin fails.
/// - The command exits with a non-zero status, indicating failure.
/// - The command’s output cannot be captured or processed.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::commands::run_sudo_command_with_stdin;
/// let content = "key=value\n".to_string();
/// let result = run_sudo_command_with_stdin("tee", &["/etc/config"], content);
/// match result {
///     Ok(()) => println!("Configuration written successfully"),
///     Err(e) => eprintln!("Failed to write configuration: {}", e),
/// }
/// ```
///
/// # Safety
/// Use caution when running commands with `sudo`, as they execute with root privileges. Ensure
/// `command`, `args`, and `stdin_content` are validated to prevent unintended system modifications.
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
        .map_err(|e| format!("Failed to spawn `{}`: {}", command, e))?;
    if let Some(mut stdin) = cmd.stdin.take() {
        stdin
            .write_all(stdin_content.as_bytes())
            .map_err(|e| format!("Failed to write to `{}` stdin: {}", command, e))?;
    }
    let output = cmd
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for `{}`: {}", command, e))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}
