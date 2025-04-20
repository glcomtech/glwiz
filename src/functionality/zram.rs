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

/// sets up zram swap configuration
pub fn zram_swap_setup() -> i8 {
    let output = Command::new("sudo")
        .arg("cp")
        .arg("../configs/zram-generator.conf")
        .arg("/etc/systemd/")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("zram {}", "swap configuration copied successfully.".green());
                return 0;
            } else {
                eprintln!(
                    "{}{}",
                    "error copying zram-generator.conf:".red(),
                    String::from_utf8_lossy(&output.stderr).red()
                );
                return 1;
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}",
                "error executing command:".red(),
                e.to_string().red()
            );
            return 2;
        }
    }
}
