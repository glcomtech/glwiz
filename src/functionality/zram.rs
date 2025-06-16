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

/// Configures ZRAM swap by copying the ZRAM generator configuration file.
///
/// Copies the ZRAM configuration file from `../configs/zram-generator.conf` to
/// `/etc/systemd/zram-generator.conf` using `cp` with sudo privileges. This sets up
/// compressed RAM-based swap to improve system performance. Logs success or failure
/// with appropriate messages.
///
/// # Returns
/// * `0` if the configuration file is copied successfully.
/// * `1` if the copy operation fails (e.g., due to permission issues or missing source file).
///
/// # Examples
/// ```
/// let result = zram_swap_setup();
/// assert_eq!(result, 0);
/// ```
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
