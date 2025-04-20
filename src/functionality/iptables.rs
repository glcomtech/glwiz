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

/// sets up iptables
pub fn iptables_file_setup() -> i8 {
    let source_path = Path::new("../configs/iptables.rules");
    let dest_path = Path::new("/etc/iptables/iptables.rules");

    match read_to_string(source_path) {
        Ok(rules) => {
            let command = "tee";
            let args = &[dest_path.as_os_str().to_str().unwrap()];

            match run_sudo_command_with_stdin(command, args, rules) {
                Ok(_) => {
                    println!("iptables.rules {}", "created successfully".green());
                    return 0;
                }
                Err(e) => {
                    eprintln!("error setting up iptables file: {}", e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("failed to read iptables rules from source file: {}", e);
            2
        }
    }
} // iptables_setup()

/// immediately sets up iptables rules
pub fn iptables_rules_setup() -> i8 {
    let rules_path = String::from("/etc/iptables/iptables.rules");
    let command = "bash";
    let args = &["-c", &format!("iptables-restore < {}", rules_path)];

    match run_sudo_command(command, args) {
        Ok(_) => {
            println!("iptables.rules {}", "set successfully".green());
            0
        }
        Err(e) => {
            eprintln!("error applying iptables rules: {}", e);
            1
        }
    }
} // iptables_rules_setup()
