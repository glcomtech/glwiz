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

/// Configures ZRAM swap to optimize system memory usage.
///
/// This function sets up ZRAM (compressed RAM-based swap) by copying a predefined configuration
/// file from `../configs/zram-generator.conf` to `/etc/systemd/zram-generator.conf`. It is part
/// of the "gnulinwiz" project’s post-installation setup to enhance system performance by providing
/// fast, compressed swap space. The function checks for the source file’s existence and prompts
/// the user to overwrite the destination if it exists, ensuring idempotent operation. It uses
/// `sudo` to write to the system directory, guaranteeing proper permissions.
///
/// # Returns
/// * `0` - The ZRAM configuration was successfully applied or skipped (user chose not to overwrite).
/// * `1` - An error occurred, such as a missing source file or failed copy operation.
///
/// # Errors
/// Returns `1` if:
/// - The source file `../configs/zram-generator.conf` does not exist.
/// - The copy operation fails due to permissions or `sudo` issues.
///
/// # Example
/// ```should_panic
/// // Requires ../configs/zram-generator.conf and sudo privileges.
/// // Use integration tests for actual validation.
/// use gnulinwiz::functionality::zram::zram_swap_setup;
/// let result = zram_swap_setup();
/// assert_eq!(result, 0); // Success if config exists and no overwrite
/// ```
/// 
/// # See Also
/// - `commands::run_sudo_command`: Used to copy the configuration file with `sudo`.
/// - `prog_fun::read_input`: Used to prompt for overwrite confirmation.
pub fn zram_swap_setup() -> i8 {
    let src = "../configs/zram-generator.conf";
    let dest = "/etc/systemd/zram-generator.conf";

    if !std::path::Path::new(src).exists() {
        eprintln!("{} Source file {} not found.", "error:".red(), src);
        return 1;
    }

    if std::path::Path::new(dest).exists() {
        println!("{} exists. Overwrite? (y/n)", dest);
        let input = super::prog_fun::read_input().trim().to_lowercase();
        if input != "y" {
            println!("ZRAM config {}.", "skipped".green());
            return 0;
        }
    }

    match run_sudo_command("cp", &[src, dest]) {
        Ok(_) => {
            println!("ZRAM {}.", "configured".green());
            0
        }
        Err(e) => {
            eprintln!("{} Failed to configure ZRAM: {}", "error:".red(), e);
            1
        }
    }
}
