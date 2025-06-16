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

use super::prog_fun::handle_error;
use colored::Colorize;
use std::process::Command;

/// Installs software packages using the `pacman` package manager.
///
/// Executes `sudo pacman -Sy` with the provided package names and the `--noconfirm` flag to
/// install software without prompting for user confirmation. Logs the command being run and
/// the outcome, including detailed output in case of failure.
///
/// # Arguments
/// * `packages` - A slice of package names to install (e.g., `["firefox", "zsh"]`).
///
/// # Returns
/// * `0` if the installation succeeds (including non-fatal warnings handled by `pacman`).
/// * `1` if the installation fails due to a command execution error or `pacman` reporting an error.
///
/// # Panics
/// Exits the program via `handle_error` if the `pacman` command cannot be executed (e.g., due to
/// permission issues or `pacman` not being found).
///
/// # Examples
/// ```
/// let result = software_setup(&["firefox", "zsh"]);
/// assert_eq!(result, 0);
/// ```
pub fn software_setup(packages: &[&str]) -> i8 {
    let mut command = Command::new("sudo");
    command
        .arg("pacman")
        .arg("-Sy")
        .args(packages)
        .arg("--noconfirm");

    println!(
        "Running command: {}{}{}",
        "sudo pacman -Sy ".green(),
        packages.join(" "),
        " --noconfirm".green()
    );

    let output = match command.output() {
        Ok(output) => output,
        Err(e) => {
            handle_error(&format!("Failed to execute pacman command: {}", e));
        }
    };

    if output.status.success() {
        println!("software {}", "installed successfully".green());
        return 0;
    } else {
        eprintln!("software installation {}", "failed".red());
        eprintln!("--- stdout ---");
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("--- stderr ---");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("--------------");
        return 1;
    }
}
