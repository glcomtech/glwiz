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

use super::commands::run_sudo_command;
use colored::Colorize;
use std::path::Path;

/// Copies a configuration file to the user's home directory.
///
/// This function copies a specified configuration file to the user's home directory,
/// preserving the original filename. It is used to set up user-specific configurations
/// such as `.zshrc` or `.vimrc`.
///
/// # Arguments
/// * `config_path` - Path to the source configuration file.
/// * `home_dir` - Path to the user's home directory.
/// * `cfg_name` - Descriptive name of the configuration (e.g., "zsh", "vim") for logging purposes.
///
/// # Returns
/// * `0` if the configuration file is copied successfully.
/// * `1` if an error occurs, such as an invalid path or file copy failure.
///
/// # Examples
/// ```
/// let result = user_config_setup("../configs/.zshrc", "/home/user", "zsh");
/// assert_eq!(result, 0);
/// ```
pub fn user_config_setup(config_path: &str, home_dir: &str, cfg_name: &str) -> i8 {
    let source = Path::new(&config_path);
    let filename = source.file_name();

    match filename {
        Some(name) => {
            let destination_path = Path::new(&home_dir).join(name);
            match std::fs::copy(config_path, &destination_path) {
                Ok(_) => {
                    println!("{} {}", cfg_name, "custom config was installed".green());
                    return 0;
                }
                Err(e) => {
                    eprintln!(
                        "error: custom config failed to install {} to '{}': {}",
                        cfg_name,
                        destination_path.display(),
                        e
                    );
                    return 1;
                }
            }
        }
        None => {
            eprintln!(
                "error: could not determine filename from path: {}",
                config_path.red()
            );
            return 1;
        }
    }
}

// Helper function to copy a file or directory as root using `cp -r` via sudo.
// Takes source path, destination path, and a description for logging.
// Returns 0 on success, 1 on failure.
fn copy_item_as_root(src: &str, dest: &str, description: &str) -> i8 {
    let args = &["-r", src, dest];

    match run_sudo_command("cp", args) {
        Ok(_) => {
            println!("{} {}", description, "created configuration".green());
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{} failed to copy '{}' to '{}': {}",
                "error:".red(),
                src,
                dest,
                e.red()
            );
            return 1;
        }
    }
}

/// Configures the root user's environment by copying configuration files from the user's home directory.
///
/// This function copies `.oh-my-zsh`, `.zshrc`, and `.vimrc` from the specified user home directory
/// to the root user's directory (`/root`) using `cp -r` with sudo privileges. It ensures the root
/// user has consistent shell and editor configurations.
///
/// # Arguments
/// * `home_dir` - Path to the user's home directory containing the source configuration files.
///
/// # Returns
/// * `0` if all configurations are copied successfully.
/// * `1` if any copy operation fails.
///
/// # Examples
/// ```
/// let result = setup_root_config("/home/user");
/// assert_eq!(result, 0);
/// ```
pub fn setup_root_config(home_dir: &str) -> i8 {
    let items_to_copy = [
        (
            format!("{}/.oh-my-zsh", home_dir),
            "/root/.oh-my-zsh".to_string(),
            "/root/.oh-my-zsh",
        ),
        (
            format!("{}/.zshrc", home_dir),
            "/root/.zshrc".to_string(),
            "/root/.zshrc",
        ),
        (
            format!("{}/.vimrc", home_dir),
            "/root/.vimrc".to_string(),
            "/root/.vimrc",
        ),
    ];

    for (src, dest, desc) in &items_to_copy {
        let status = copy_item_as_root(src.as_str(), dest.as_str(), desc);
        if status != 0 {
            return 1;
        }
    }

    return 0;
}
