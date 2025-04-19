/*
gnulinwiz AKA GNU/Linux Config Wizard: The ultimate post-installation setup assistant for Linux,
streamlining your configuration process with ease and precision.

Copyright (C) 2025  Andrew Kushyk

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::commands::run_sudo_command;
use colored::Colorize;
use std::path::Path;

/// sets up config files in home directory
pub fn user_config_setup(config_path: String, home_dir: String, cfg_name: &str) -> i8 {
    let source = Path::new(&config_path);
    let filename = source.file_name();

    match filename {
        Some(name) => {
            let destination_path = Path::new(&home_dir).join(name);
            match std::fs::copy(config_path, &destination_path) {
                Ok(_) => {
                    println!("{} {}", cfg_name, "custom config was installed".green());
                    return 0;
                }
                Err(e) => {
                    eprintln!(
                        "error: custom config failed to install {} to '{}': {}",
                        cfg_name,
                        destination_path.display(),
                        e
                    );
                    return 1;
                }
            }
        }
        None => {
            eprintln!(
                "error: could not determine filename from path: {}",
                config_path.red()
            );
            return 1;
        }
    }
} // user_config_setup

/// sets up root config in /root directory
pub fn setup_root_config(home_dir: String) -> i8 {
    let oh_my_zsh_src = format!("{}{}", home_dir, "/.oh-my-zsh");
    let oh_my_zsh_dest = String::from("/root/.oh-my-zsh");
    let zshrc_src = format!("{}{}", home_dir, "/.zshrc");
    let zshrc_dest = String::from("/root/.zshrc");
    let vimrc_src = format!("{}{}", home_dir, "/.vimrc");
    let vimrc_dest = String::from("/root/.vimrc");

    // Create symbolic link for .oh-my-zsh
    match run_sudo_command(
        "cp",
        &["-r", oh_my_zsh_src.as_str(), oh_my_zsh_dest.as_str()],
    ) {
        Ok(_) => println!("/root/.oh-my-zsh {}", "created configuration".green()),
        Err(e) => {
            eprintln!(
                "{}{}",
                "error creating configuration /root/.oh-my-zsh:".red(),
                e.red()
            );
            return 1;
        }
    }

    // Create symbolic link for .zshrc
    match run_sudo_command("cp", &["-r", zshrc_src.as_str(), zshrc_dest.as_str()]) {
        Ok(_) => println!("/root/.zshrc {}", "created configuration".green()),
        Err(e) => {
            eprintln!(
                "{}{}",
                "error creating configuration /root/.zshrc:".red(),
                e.red()
            );
            return 2;
        }
    }

    // Create symbolic link for .vimrc
    match run_sudo_command("cp", &["-r", vimrc_src.as_str(), vimrc_dest.as_str()]) {
        Ok(_) => println!("/root/.vimrc {}", "created configuration".green()),
        Err(e) => {
            eprintln!(
                "{}{}",
                "error creating configuration /root/.vimrc:".red(),
                e.red()
            );
            return 3;
        }
    }

    return 0;
} // setup_root_config
