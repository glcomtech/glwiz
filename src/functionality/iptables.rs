/*
 * GLWiz - The ultimate post-installation setup assistant for GNU/Linux popular distros,
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

use super::commands::{run_sudo_command, run_sudo_command_with_stdin};
use colored::Colorize;
use std::fs;

/// Sets up the iptables configuration file for network security.
///
/// This function copies a predefined iptables rules file from `../configs/iptables.rules` to
/// `/etc/iptables/iptables.rules`, ensuring a secure firewall configuration in the "glwiz" project.
/// It checks for the source file’s existence and prompts the user to overwrite the destination if it
/// exists, making the operation idempotent. The function uses `sudo` to write to the system directory,
/// ensuring proper permissions. It is part of the post-installation setup to enhance network security.
///
/// # Returns
/// * `0` - The rules file was successfully created or skipped (user chose not to overwrite).
/// * `1` - An error occurred, such as a missing source file, read failure, or write error.
///
/// # Errors
/// Returns `1` if:
/// - The source file `../configs/iptables.rules` does not exist.
/// - Reading the source file fails due to permissions or I/O errors.
/// - Writing to `/etc/iptables/iptables.rules` fails due to permissions or `sudo` issues.
///
/// # Example
/// ```should_panic
/// // Requires ../configs/iptables.rules and sudo privileges.
/// use glwiz::functionality::iptables::iptables_file_setup;
/// let result = iptables_file_setup();
/// assert_eq!(result, 0);
/// ```
///
/// # See Also
/// - `commands::run_sudo_command_with_stdin`: Used to write the rules file with `sudo`.
/// - `prog_fun::read_input`: Used to prompt for overwrite confirmation.
/// - `iptables_rules_setup`: Applies the configured rules.
pub fn iptables_file_setup() -> i8 {
    let src = "../configs/iptables.rules";
    let dest = "/etc/iptables/iptables.rules";

    if !std::path::Path::new(src).exists() {
        eprintln!("{} Source file {} not found.", "error:".red(), src);
        return 1;
    }

    let rules = match fs::read_to_string(src) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{} Failed to read {}: {}", "error:".red(), src, e);
            return 1;
        }
    };

    if std::path::Path::new(dest).exists() {
        println!("{} exists. Overwrite? (y/n)", dest);
        let input = super::prog_fun::read_input().trim().to_lowercase();
        if input != "y" {
            println!("iptables rules {}.", "skipped".green());
            return 0;
        }
    }

    match run_sudo_command_with_stdin("tee", &[dest], rules) {
        Ok(_) => {
            println!("iptables rules {}.", "created".green());
            0
        }
        Err(e) => {
            eprintln!("{} Failed to write iptables rules: {}", "error:".red(), e);
            1
        }
    }
}

/// Applies the configured iptables rules to enforce network security.
///
/// This function uses `iptables-restore` to apply the rules stored in `/etc/iptables/iptables.rules`,
/// activating the firewall configuration set up by `iptables_file_setup`. It executes the command
/// with `sudo` to ensure proper permissions and is part of the "glwiz" project’s post-installation
/// setup to secure the system’s network. The function logs success or failure with descriptive messages.
///
/// # Returns
/// * `0` - The iptables rules were successfully applied.
/// * `1` - An error occurred, such as a missing rules file or `sudo` command failure.
///
/// # Errors
/// Returns `1` if:
/// - The rules file `/etc/iptables/iptables.rules` does not exist or is invalid.
/// - The `iptables-restore` command fails due to permissions or syntax errors in the rules.
///
/// # Example
/// ```
/// use glwiz::functionality::iptables::iptables_rules_setup;
/// let result = iptables_rules_setup();
/// assert_eq!(result, 0); // Rules applied successfully
/// ```
///
/// # See Also
/// - `commands::run_sudo_command`: Used to execute `iptables-restore` with `sudo`.
/// - `iptables_file_setup`: Sets up the rules file before application.
pub fn iptables_rules_setup() -> i8 {
    let rules_path = "/etc/iptables/iptables.rules";

    match run_sudo_command(
        "bash",
        &["-c", &format!("iptables-restore < {}", rules_path)],
    ) {
        Ok(_) => {
            println!("iptables rules {}.", "applied".green());
            0
        }
        Err(e) => {
            eprintln!("{} Failed to apply iptables rules: {}", "error:".red(), e);
            1
        }
    }
}
