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

use super::commands::{run_sudo_command, run_user_command};
use colored::Colorize;
use std::process::{Command, Stdio};

/// Changes the default shell to Zsh for the specified user.
///
/// Uses the `chsh` command with sudo privileges to set `/usr/bin/zsh` as the default shell
/// for the given user. Logs success or failure with appropriate messages.
///
/// # Arguments
/// * `name` - The username for which to change the default shell.
///
/// # Returns
/// * `0` if the shell is changed successfully.
/// * `1` if the command fails or an error occurs.
///
/// # Examples
/// ```
/// let result = change_def_shell("user");
/// assert_eq!(result, 0);
/// ```
pub fn change_def_shell(name: &str) -> i8 {
    let command = "chsh";
    let args = &["-s", "/usr/bin/zsh", name];

    match run_sudo_command(command, args) {
        Ok(_) => {
            println!("zsh {}{}", "shell set successfully for ".green(), name);
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{} failed to change default shell for user '{}': {}",
                "error:".red(),
                name,
                e.red()
            );
            return 1;
        }
    }
}

/// Installs Oh My Zsh by downloading and executing its installation script.
///
/// Downloads the Oh My Zsh installation script using `curl` and pipes it to `bash` for execution.
/// Handles errors during process spawning, piping, or script execution, and logs detailed output
/// in case of failure.
///
/// # Returns
/// * `0` if Oh My Zsh is installed successfully.
/// * `1` if any error occurs during the installation process.
///
/// # Examples
/// ```
/// let result = install_omz();
/// assert_eq!(result, 0);
/// ```
pub fn install_omz() -> i8 {
    let mut curl_cmd = Command::new("curl");
    curl_cmd
        .args(&[
            "-fsSL",
            "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh",
        ])
        .stdout(Stdio::piped());

    let mut curl_process = match curl_cmd.spawn() {
        Ok(process) => process,
        Err(e) => {
            eprintln!("{} failed to spawn curl command: {}", "error:".red(), e);
            return 1;
        }
    };

    let curl_stdout = match curl_process.stdout.take() {
        Some(stdout) => stdout,
        None => {
            eprintln!("{}", "error: failed to capture curl stdout pipe".red());
            let _ = curl_process.wait();
            return 1;
        }
    };

    let mut bash_cmd = Command::new("bash");
    bash_cmd
        .stdin(Stdio::from(curl_stdout))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut bash_process = match bash_cmd.spawn() {
        Ok(process) => process,
        Err(e) => {
            eprintln!("{} failed to spawn bash command: {}", "error:".red(), e);
            let _ = curl_process.kill();
            let _ = curl_process.wait();
            return 1;
        }
    };

    let curl_status = match curl_process.wait() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("{} failed to wait for curl command: {}", "error:".red(), e);
            let _ = bash_process.kill();
            let _ = bash_process.wait();
            return 1;
        }
    };

    if !curl_status.success() {
        eprintln!(
            "{} curl command failed with status: {}",
            "error:".red(),
            curl_status
                .code()
                .map_or_else(|| "terminated by signal".into(), |c| c.to_string())
        );
        let _ = bash_process.kill();
        let _ = bash_process.wait();
        return 1;
    }

    let output = match bash_process.wait_with_output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{} failed to wait for bash command: {}", "error:".red(), e);
            return 1;
        }
    };

    if output.status.success() {
        println!("oh-my-zsh {}", "installed successfully".green());
        return 0;
    } else {
        eprintln!(
            "{} oh-my-zsh installation script failed with status: {}",
            "error:".red(),
            output
                .status
                .code()
                .map_or_else(|| "terminated by signal".into(), |c| c.to_string())
        );
        if !output.stdout.is_empty() {
            eprintln!("--- Script stdout ---");
            eprintln!("{}", String::from_utf8_lossy(&output.stdout).trim());
        }
        if !output.stderr.is_empty() {
            eprintln!("--- Script stderr ---");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr).trim());
        }
        if output.stdout.is_empty() && output.stderr.is_empty() {
            eprintln!("(No script output captured)");
        }

        return 1;
    }
}

// Helper function to clone a Zsh plugin into the Oh My Zsh custom plugins directory.
// Clones the specified repository into the user's Oh My Zsh plugins directory using `git`.
fn install_zsh_plugin(home_dir: &str, plugin_name: &str, repo_url: &str) -> i8 {
    let zsh_custom_path = format!("{}/.oh-my-zsh/custom/plugins/{}", home_dir, plugin_name);
    let args = &["clone", repo_url, &zsh_custom_path];

    println!("Cloning {}...", plugin_name.green());

    match run_user_command("git", args) {
        Ok(_) => {
            println!("{} {}", plugin_name, "installed successfully".green());
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{} failed to clone '{}' from '{}' into '{}': {}",
                "error:".red(),
                plugin_name,
                repo_url,
                zsh_custom_path,
                e.red(),
            );
            return 1;
        }
    }
}

/// Installs the Zsh Autosuggestions plugin for the specified user's Oh My Zsh setup.
///
/// Clones the Zsh Autosuggestions plugin from its GitHub repository into the user's
/// Oh My Zsh custom plugins directory.
///
/// # Arguments
/// * `home_dir` - The user's home directory where Oh My Zsh is installed.
///
/// # Returns
/// * `0` if the plugin is installed successfully.
/// * `1` if the plugin installation fails (e.g., due to `git` errors).
///
/// # Examples
/// ```
/// let result = install_zsh_autosuggestions("/home/user");
/// assert_eq!(result, 0);
/// ```
pub fn install_zsh_autosuggestions(home_dir: &str) -> i8 {
    install_zsh_plugin(
        home_dir,
        "zsh-autosuggestions",
        "https://github.com/zsh-users/zsh-autosuggestions",
    )
}

/// Installs the Zsh Syntax Highlighting plugin for the specified user's Oh My Zsh setup.
///
/// Clones the Zsh Syntax Highlighting plugin from its GitHub repository into the user's
/// Oh My Zsh custom plugins directory.
///
/// # Arguments
/// * `home_dir` - The user's home directory where Oh My Zsh is installed.
///
/// # Returns
/// * `0` if the plugin is installed successfully.
/// * `1` if the plugin installation fails (e.g., due to `git` errors).
///
/// # Examples
/// ```
/// let result = install_zsh_syntax_highlighting("/home/user");
/// assert_eq!(result, 0);
/// ```
pub fn install_zsh_syntax_highlighting(home_dir: &str) -> i8 {
    install_zsh_plugin(
        home_dir,
        "zsh-syntax-highlighting",
        "https://github.com/zsh-users/zsh-syntax-highlighting.git",
    )
}
