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
use std::process::Command;
use super::prog_fun::handle_error;

/// installs software using pacman
/// Takes a slice of string slices for package names.
/// Returns 0 on success (including warnings treated by pacman as non-fatal),
/// 1 on definitive failure (command failed to run or pacman reported an error).
pub fn software_setup(packages: &[&str]) -> i8 {
    let mut command = Command::new("sudo");
    command
        .arg("pacman")
        .arg("-Sy")
        .args(packages)
        .arg("--noconfirm");

    println!("{}{}{}", "Running command: sudo pacman -Sy ".green(), packages.join(" "), " --noconfirm".green());

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
