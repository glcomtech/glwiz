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

/// Changes the default shell to Zsh for a specified user.
///
/// This function sets Zsh as the default shell for a user by executing the `chsh` command with
/// `sudo` privileges. It is part of the "gnulinwiz" project’s post-installation setup to provide
/// an enhanced shell experience. The function logs success or failure and is used for both
/// regular users and the root user.
///
/// # Arguments
/// * `name` - The username for which to set Zsh as the default shell (e.g., `"user"`, `"root"`).
///
/// # Returns
/// * `0` - The shell was successfully changed to Zsh.
/// * `1` - An error occurred, such as a failed `chsh` command or invalid username.
///
/// # Errors
/// Returns `1` if:
/// - The `chsh` command fails due to permissions, invalid shell path, or non-existent user.
/// - The `sudo` execution encounters an error (e.g., `sudo` not installed).
///
/// # Example
/// ```
/// use gnulinwiz::functionality::shell::change_def_shell;
/// let result = change_def_shell("user");
/// assert_eq!(result, 0); // Zsh set successfully for user
/// ```
///
/// # See Also
/// - `commands::run_sudo_command`: Used to execute `chsh` with `sudo`.
/// - `configs::user_config_setup`: Configures Zsh settings after shell change.
pub fn change_def_shell(name: &str) -> i8 {
    match run_sudo_command("chsh", &["-s", "/usr/bin/zsh", name]) {
        Ok(_) => {
            println!("Zsh set for {}.", name.green());
            0
        }
        Err(e) => {
            eprintln!("{} Failed to set Zsh for {}: {}", "error:".red(), name, e);
            1
        }
    }
}

/// Installs Oh My Zsh to enhance the Zsh shell experience.
///
/// This function downloads and installs Oh My Zsh by piping the official installation script from
/// a remote URL through `curl` to `bash`. It checks if Oh My Zsh is already installed to avoid
/// redundant operations, ensuring idempotency. The function is part of the "gnulinwiz" project’s
/// post-installation setup to provide a customizable and feature-rich shell environment.
///
/// # Returns
/// * `0` - Oh My Zsh was successfully installed or already present.
/// * `1` - An error occurred during installation, such as a failed `curl` or `bash` command.
///
/// # Errors
/// Returns `1` if:
/// - The `curl` command fails to download the installation script.
/// - Capturing `curl`’s stdout fails.
/// - The `bash` command fails to execute the script or returns a non-zero exit status.
///
/// # Example
/// ```should_panic
/// // Requires network access and curl/bash.
/// use gnulinwiz::functionality::shell::install_omz;
/// let result = install_omz();
/// assert_eq!(result, 0);
/// ```
///
/// # See Also
/// - `install_zsh_autosuggestions`: Installs a complementary Zsh plugin.
/// - `install_zsh_syntax_highlighting`: Installs another Zsh plugin.
pub fn install_omz() -> i8 {
    if std::path::Path::new("~/.oh-my-zsh").exists() {
        println!("Oh My Zsh already installed.");
        return 0;
    }

    let mut curl_cmd = Command::new("curl");
    curl_cmd
        .args(&[
            "-fsSL",
            "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh",
        ])
        .stdout(Stdio::piped());

    let curl_process = match curl_cmd.spawn() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Failed to run curl: {}", "error:".red(), e);
            return 1;
        }
    };

    let curl_stdout = match curl_process.stdout {
        Some(s) => s,
        None => {
            eprintln!("{} Failed to capture curl stdout.", "error:".red());
            return 1;
        }
    };

    let mut bash_cmd = Command::new("bash");
    bash_cmd.stdin(Stdio::from(curl_stdout));
    match bash_cmd.output() {
        Ok(output) if output.status.success() => {
            println!("Oh My Zsh {}.", "installed".green());
            0
        }
        Ok(output) => {
            eprintln!(
                "{} Oh My Zsh failed:\nstdout: {}\nstderr: {}",
                "error:".red(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            1
        }
        Err(e) => {
            eprintln!("{} Failed to run bash: {}", "error:".red(), e);
            1
        }
    }
}

// Installs a Zsh plugin by cloning a Git repository.
//
// This private helper function clones a specified Zsh plugin repository into the Oh My Zsh
// custom plugins directory. It checks if the plugin is already installed to avoid redundant
// cloning, ensuring idempotency. The function is used by `install_zsh_autosuggestions` and
// `install_zsh_syntax_highlighting` to install specific plugins.
//
// Arguments:
// * `home_dir` - The user’s home directory (e.g., "/home/user").
// * `plugin_name` - The name of the plugin (e.g., "zsh-autosuggestions").
// * `repo_url` - The Git repository URL for the plugin.
//
// Returns:
// * `0` - The plugin was successfully installed or already present.
// * `1` - An error occurred during the Git clone operation.
fn install_zsh_plugin(home_dir: &str, plugin_name: &str, repo_url: &str) -> i8 {
    let path = format!("{}/.oh-my-zsh/custom/plugins/{}", home_dir, plugin_name);
    if std::path::Path::new(&path).exists() {
        println!("{} already installed.", plugin_name);
        return 0;
    }

    match run_user_command("git", &["clone", repo_url, &path]) {
        Ok(_) => {
            println!("{} {}.", plugin_name, "installed".green());
            0
        }
        Err(e) => {
            eprintln!(
                "{} Failed to install {}: {}",
                "error:".red(),
                plugin_name,
                e
            );
            1
        }
    }
}

/// Installs the Zsh Autosuggestions plugin for enhanced shell interaction.
///
/// This function installs the Zsh Autosuggestions plugin by cloning its Git repository into the
/// Oh My Zsh custom plugins directory. It ensures idempotency by checking for existing installations
/// and is part of the "gnulinwiz" project’s post-installation setup to improve Zsh usability with
/// command suggestions based on history.
///
/// # Arguments
/// * `home_dir` - The user’s home directory where Oh My Zsh is installed (e.g., `"/home/user"`).
///
/// # Returns
/// * `0` - The plugin was successfully installed or already present.
/// * `1` - An error occurred during the Git clone operation.
///
/// # Errors
/// Returns `1` if the `git clone` command fails due to network issues, permissions, or invalid URLs.
///
/// # Example
/// ```should_panic
/// // Requires git and network access.
/// use gnulinwiz::functionality::shell::install_zsh_autosuggestions;
/// let result = install_zsh_autosuggestions("/home/user");
/// assert_eq!(result, 0);
/// ```
///
/// # See Also
/// - `install_zsh_plugin`: The helper function performing the installation.
/// - `install_omz`: Installs Oh My Zsh, required for this plugin.
pub fn install_zsh_autosuggestions(home_dir: &str) -> i8 {
    install_zsh_plugin(
        home_dir,
        "zsh-autosuggestions",
        "https://github.com/zsh-users/zsh-autosuggestions",
    )
}

/// Installs the Zsh Syntax Highlighting plugin for improved shell readability.
///
/// This function installs the Zsh Syntax Highlighting plugin by cloning its Git repository into the
/// Oh My Zsh custom plugins directory. It ensures idempotency by checking for existing installations
/// and is part of the "gnulinwiz" project’s post-installation setup to enhance Zsh with syntax
/// highlighting for commands and arguments.
///
/// # Arguments
/// * `home_dir` - The user’s home directory where Oh My Zsh is installed (e.g., `"/home/user"`).
///
/// # Returns
/// * `0` - The plugin was successfully installed or already present.
/// * `1` - An error occurred during the Git clone operation.
///
/// # Errors
/// Returns `1` if the `git clone` command fails due to network issues, permissions, or invalid URLs.
///
/// # Example
/// ```should_panic
/// // Requires git and network access.
/// use gnulinwiz::functionality::shell::install_zsh_syntax_highlighting;
/// let result = install_zsh_syntax_highlighting("/home/user");
/// assert_eq!(result, 0); // Plugin installed or already present
/// ```
///
/// # See Also
/// - `install_zsh_plugin`: The helper function performing the installation.
/// - `install_omz`: Installs Oh My Zsh, required for this plugin.
pub fn install_zsh_syntax_highlighting(home_dir: &str) -> i8 {
    install_zsh_plugin(
        home_dir,
        "zsh-syntax-highlighting",
        "https://github.com/zsh-users/zsh-syntax-highlighting.git",
    )
}
