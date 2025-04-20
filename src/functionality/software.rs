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

/// installs software
pub fn software_setup(packages: &[String]) -> i8 {
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-Sy")
        .args(packages.iter().map(|s| s.as_str()))
        .arg("--noconfirm")
        .output()
        .expect("failed to install necessary software.");

    if output.status.success() {
        println!("software {}", "installed successfully".green());
        return 0;
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.contains("error:") && !stderr.contains("failed to download") {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
            eprintln!("{}", "warnings encountered during installation.".yellow());
            return 0;
        } else {
            eprintln!("{}{}", "error:\n".red(), stderr.red());
            return 1;
        }
    }
} // software_setup()
