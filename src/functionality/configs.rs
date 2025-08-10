/*
 * GLWiz - The ultimate post-installation setup assistant for GNU/Linux popular disros,
 * streamlining your configuration process with ease and precision.
 * 
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::commands::run_sudo_command;
use colored::Colorize;
use std::path::Path;

/// Copies a configuration file to the user's home directory, ensuring idempotent operation.
///
/// This function copies a specified configuration file (e.g., `.zshrc`, `.vimrc`) to the user’s
/// home directory, creating a consistent user environment in the "glwiz" project. It checks
/// for existing files at the destination and prompts the user to overwrite them, preventing
/// unintended modifications. The function is used for setting up user-specific configurations
/// like Zsh and Vim settings during post-installation setup.
///
/// # Arguments
/// * `config_path` - The path to the source configuration file (e.g., `"../configs/.zshrc"`).
/// * `home_dir` - The user’s home directory where the file will be copied (e.g., `"/home/user"`).
/// * `cfg_name` - A descriptive name for the configuration (e.g., `"zsh"`, `"vim"`) used in logs.
///
/// # Returns
/// * `0` - The configuration was successfully copied or skipped (user chose not to overwrite).
/// * `1` - An error occurred, such as an invalid source path or file copy failure.
///
/// # Errors
/// Returns `1` if:
/// - The source path is invalid or does not contain a file name.
/// - The file copy operation fails due to permissions or other I/O errors.
///
/// # Example
/// ```should_panic
/// // This example requires a valid ../configs/.zshrc file and write permissions.
/// // For actual testing, use integration tests with a mocked file system.
/// use glwiz::functionality::configs::user_config_setup;
/// let result = user_config_setup("../configs/.zshrc", "/home/user", "zsh");
/// assert_eq!(result, 0); // Success if files exist and no overwrite prompt
/// ```
///
/// # See Also
/// - `prog_fun::read_input`: Used to prompt the user for overwrite confirmation.
/// - `setup_root_config`: For configuring the root user’s environment.
pub fn user_config_setup(config_path: &str, home_dir: &str, cfg_name: &str) -> i8 {
    let source = Path::new(config_path);
    let filename = match source.file_name() {
        Some(name) => name,
        None => {
            eprintln!("{} Invalid path: {}", "error:".red(), config_path);
            return 1;
        }
    };

    let dest_path = Path::new(home_dir).join(filename);
    if dest_path.exists() {
        println!(
            "{} exists. {} (y/n)",
            dest_path.display(),
            "Overwrite?".yellow()
        );
        let input = super::prog_fun::read_input().trim().to_lowercase();
        if input != "y" {
            println!("{} Skipped.", cfg_name.green());
            return 0;
        }
    }

    match std::fs::copy(config_path, &dest_path) {
        Ok(_) => {
            println!("{} {}.", cfg_name, "installed".green());
            0
        }
        Err(e) => {
            eprintln!("{} Failed to install {}: {}", "error:".red(), cfg_name, e);
            1
        }
    }
}

// Copies a file or directory to a system location using root privileges.
//
// This private helper function executes a `cp` command with `sudo` to copy a file or directory
// from a source to a destination, typically for root-owned locations like `/root`. It is used
// by `setup_root_config` to set up root user configurations. The function logs success or failure
// with descriptive messages.
//
// Arguments:
// * `src` - The source path of the file or directory.
// * `dest` - The destination path for the copy.
// * `description` - A descriptive name for the item being copied (e.g., "Root Zsh config").
//
// Returns:
// * `0` - The copy operation succeeded.
// * `1` - The copy operation failed, with an error logged to stderr.
fn copy_item_as_root(src: &str, dest: &str, description: &str) -> i8 {
    match run_sudo_command("cp", &["-r", src, dest]) {
        Ok(_) => {
            println!("{} {}.", description, "created".green());
            0
        }
        Err(e) => {
            eprintln!("{} Failed to copy {}: {}", "error:".red(), description, e);
            1
        }
    }
}

/// Configures the root user’s environment by copying user configurations to root directories.
///
/// This function copies essential configuration files and directories (e.g., `.oh-my-zsh`, `.zshrc`,
/// `.vimrc`) from the user’s home directory to the root user’s environment (e.g., `/root`). It uses
/// `sudo` to perform the copy operations, ensuring root-owned files are updated correctly. The function
/// is part of the "glwiz" project’s post-installation setup to provide a consistent root environment.
///
/// # Arguments
/// * `home_dir` - The user’s home directory containing the source configurations (e.g., `"/home/user"`).
///
/// # Returns
/// * `0` - All configurations were successfully copied.
/// * `1` - An error occurred during one or more copy operations.
///
/// # Errors
/// Returns `1` if any copy operation fails due to:
/// - Insufficient permissions or invalid paths.
/// - Errors in the `sudo` command execution.
///
/// # Example
/// ```
/// use glwiz::functionality::configs::setup_root_config;
/// let result = setup_root_config("/home/user");
/// assert_eq!(result, 0); // Root configurations copied successfully
/// ```
///
/// # See Also
/// - `commands::run_sudo_command`: Used for executing copy operations with root privileges.
/// - `user_config_setup`: For setting up user-specific configurations.
pub fn setup_root_config(home_dir: &str) -> i8 {
    let items = [
        (
            format!("{}/.oh-my-zsh", home_dir),
            "/root/.oh-my-zsh",
            "Root Oh My Zsh",
        ),
        (
            format!("{}/.zshrc", home_dir),
            "/root/.zshrc",
            "Root Zsh config",
        ),
        (
            format!("{}/.vimrc", home_dir),
            "/root/.vimrc",
            "Root Vim config",
        ),
    ];

    for (src, dest, desc) in items.iter() {
        if copy_item_as_root(src, dest, desc) != 0 {
            return 1;
        }
    }

    0
}
