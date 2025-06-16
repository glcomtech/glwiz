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

/// Core functionality for the GNU/Linux Config Wizard, providing tools for system configuration.
pub mod functionality;

use colored::Colorize;
use functionality::{
    configs::{setup_root_config, user_config_setup},
    env::get_env_var,
    iptables::{iptables_file_setup, iptables_rules_setup},
    prog_fun::{
        check_sw_install_type, default_sw_package, handle_error, print_license_info,
        print_setup_status_success, set_sw_list, validate_root_priviliges,
    },
    shell::{
        change_def_shell, install_omz, install_zsh_autosuggestions, install_zsh_syntax_highlighting,
    },
    software::software_setup,
    task::validate_task_status,
    user_cfg::UserCfg,
    zram::zram_swap_setup,
};

/// Performs a default post-installation setup for a GNU/Linux system.
///
/// This function orchestrates the configuration of essential system components, including:
/// - Displaying license information
/// - Validating root privileges
/// - Configuring user and root environment settings
/// - Setting up the iptables firewall
/// - Installing specified or default software packages
/// - Configuring the Zsh shell with plugins
/// - Setting up Vim and Zsh configurations for both user and root
/// - Enabling ZRAM swap
///
/// The function validates each task and halts on errors, ensuring a robust setup process.
/// On successful completion, it prints a success message.
///
/// # Panics
/// Panics if critical environment variables (`USER` or `HOME`) are not set, or if any setup task fails.
pub fn gnu_linux_default_setup() {
    // prints license info
    print_license_info();

    // validates root priviliges
    validate_root_priviliges();

    // sets up gnu/linux username and home path
    let mut user_cfg = UserCfg::new();
    let user_name_option = get_env_var("USER");
    let user_name = match user_name_option {
        Some(name) => name,
        None => {
            handle_error("setup cannot continue without the USER environment variable.");
        }
    };
    let home_dir_option = get_env_var("HOME");
    let home_dir = match home_dir_option {
        Some(dir) => dir, // Got the String, unwrap it
        None => {
            handle_error("setup cannot continue without the USER environment variable.");
        }
    };
    user_cfg.set_name(user_name.as_str());
    user_cfg.set_home(home_dir.as_str());
    println!("username: {}", user_cfg.get_name().green());
    println!("home location: {}", user_cfg.get_home().green());

    // sets up iptables firewall and initializes rules
    validate_task_status(iptables_file_setup());
    validate_task_status(iptables_rules_setup());

    // sets up a list of sw to download
    validate_task_status(if check_sw_install_type() {
        let package_strings: Vec<String> = set_sw_list();
        let package_slices: Vec<&str> = package_strings.iter().map(|s| s.as_str()).collect();
        software_setup(&package_slices)
    } else {
        software_setup(default_sw_package())
    });

    validate_task_status(change_def_shell(user_cfg.get_name()));
    validate_task_status(change_def_shell("root"));

    // sets up zsh shell
    validate_task_status(install_omz());
    validate_task_status(install_zsh_autosuggestions(user_cfg.get_home()));
    validate_task_status(install_zsh_syntax_highlighting(user_cfg.get_home()));
    validate_task_status(user_config_setup(
        "../configs/.zshrc",
        user_cfg.get_home(),
        "zsh",
    ));

    // sets up vim configuration
    validate_task_status(user_config_setup(
        "../configs/.vimrc",
        user_cfg.get_home(),
        "vim",
    ));

    // sets up zsh, its plugins, .vimrc and .zshrc for root user
    validate_task_status(setup_root_config(user_cfg.get_home()));

    // sets up zram swap
    validate_task_status(zram_swap_setup());

    // prints status if no errors occured
    print_setup_status_success();
}
