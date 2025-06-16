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

use super::commands::{run_sudo_command, run_sudo_command_with_stdin};
use colored::Colorize;
use std::{fs::read_to_string, path::Path};

/// Sets up the iptables configuration file by copying rules from a source to a destination.
///
/// This function reads iptables rules from a source file (`../configs/iptables.rules`) and writes
/// them to the system location (`/etc/iptables/iptables.rules`) using `tee` with sudo privileges.
/// It ensures the destination path is valid UTF-8 and handles file reading and writing errors.
///
/// # Returns
/// * `0` if the iptables rules file is successfully created.
/// * `1` if the destination path is invalid or writing fails.
/// * `2` if the source file cannot be read.
///
/// # Examples
/// ```
/// let result = iptables_file_setup();
/// assert_eq!(result, 0);
/// ```
pub fn iptables_file_setup() -> i8 {
    let source_path = Path::new("../configs/iptables.rules");
    let dest_path = Path::new("/etc/iptables/iptables.rules");

    let rules = match read_to_string(source_path) {
        Ok(rules_content) => rules_content,
        Err(e) => {
            eprintln!(
                "{} failed to read iptables rules from source file '{}': {}",
                "error:".red(),
                source_path.display(),
                e
            );
            return 2;
        }
    };

    let command = "tee";

    let dest_path_str = match dest_path.to_str() {
        Some(s) => s,
        None => {
            eprintln!(
                "{} destination path '{}' is not valid UTF-8.",
                "error:".red(),
                dest_path.display()
            );
            return 1;
        }
    };

    let args = &[dest_path_str];

    match run_sudo_command_with_stdin(command, args, rules) {
        Ok(()) => {
            println!("iptables.rules {}", "created successfully".green());
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{} failed to write iptables rules to '{}': {}",
                "error:".red(),
                dest_path.display(),
                e.red()
            );
            return 1;
        }
    }
}

/// Applies iptables rules from the system configuration file.
///
/// This function uses `iptables-restore` to immediately apply the rules stored in
/// `/etc/iptables/iptables.rules` by executing a bash command with sudo privileges.
/// It logs success or failure with appropriate messages.
///
/// # Returns
/// * `0` if the iptables rules are applied successfully.
/// * `1` if applying the rules fails.
///
/// # Examples
/// ```
/// let result = iptables_rules_setup();
/// assert_eq!(result, 0);
/// ```
pub fn iptables_rules_setup() -> i8 {
    let rules_path = String::from("/etc/iptables/iptables.rules");
    let command = "bash";
    let args = &["-c", &format!("iptables-restore < {}", rules_path)];

    match run_sudo_command(command, args) {
        Ok(_) => {
            println!("iptables.rules {}", "set successfully".green());
            return 0;
        }
        Err(e) => {
            eprintln!("error applying iptables rules: {}", e);
            return 1;
        }
    }
}
