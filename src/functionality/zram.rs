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

/// sets up zram swap configuration by copying the generator file
pub fn zram_swap_setup() -> i8 {
    let source_config_path = "../configs/zram-generator.conf";
    let destination_path = "/etc/systemd/zram-generator.conf";

    let result = run_sudo_command("cp", &[source_config_path, destination_path]);

    match result {
        Ok(()) => {
            println!("zram {}", "swap configuration copied successfully.".green());
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{} failed to copy zram-generator.conf: {}",
                "error:".red(),
                e.red()
            );
            return 1;
        }
    }
}
