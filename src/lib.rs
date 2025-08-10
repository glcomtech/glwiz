/*
 * GLWiz - The ultimate post-installation setup assistant for GNU/Linux popular disros,
 * streamlining your configuration process with ease and precision.
 * 
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/// The core module for the glwiz, containing utilities for system configuration.
///
/// This module organizes functionality into submodules for tasks such as software installation,
/// shell configuration, iptables setup, and ZRAM swap management. Each submodule provides
/// specific tools to automate post-installation setup for GNU/Linux systems, ensuring a
/// streamlined and user-friendly experience.
///
/// # Example
/// ```
/// use glwiz::functionality::zram::zram_swap_setup;
/// let result = zram_swap_setup();
/// assert_eq!(result, 0); // Successful ZRAM configuration
/// ```
pub mod functionality;

use colored::Colorize;
use functionality::{
    configs::{setup_root_config, user_config_setup},
    env::get_env_var,
    iptables::{iptables_file_setup, iptables_rules_setup},
    prog_fun::{
        check_sw_install_type, default_sw_package, print_license_info, print_setup_status_success,
        set_sw_list, validate_root_priviliges,
    },
    shell::{
        change_def_shell, install_omz, install_zsh_autosuggestions, install_zsh_syntax_highlighting,
    },
    software::software_setup,
    task::{validate_task_statuses, TaskResult},
    user_cfg::UserCfg,
    zram::zram_swap_setup,
};

/// Orchestrates a default post-installation setup for a GNU/Linux system.
///
/// This function automates the configuration of essential system components, including:
/// - Displaying the GNU General Public License information.
/// - Validating root privileges (with an option to allow root execution).
/// - Setting up user and root environments.
/// - Configuring iptables firewall rules.
/// - Installing software (default or user-specified packages).
/// - Setting Zsh as the default shell with plugins (Oh My Zsh, autosuggestions, syntax highlighting).
/// - Installing Vim configuration.
/// - Configuring ZRAM swap for improved performance.
///
/// Tasks are executed sequentially, with results collected for comprehensive error reporting.
/// The setup is designed to be idempotent where possible, checking for existing configurations
/// to avoid redundant operations.
///
/// # Arguments
/// * `allow_root` - Enables execution with root privileges if `true`. If `false`, the program
///   exits if run as root unless explicitly allowed.
///
/// # Returns
/// * `Ok(())` - All tasks completed successfully.
/// * `Err(String)` - A summary of failed tasks, with details logged to stderr.
///
/// # Errors
/// Returns an error if:
/// - Environment variables (`USER`, `HOME`) are unset.
/// - Any task (e.g., software installation, iptables setup) fails.
/// - Root privileges are required but not allowed.
///
/// # Example
/// ```
/// let result = glwiz::gnu_linux_default_setup(true);
/// match result {
///     Ok(()) => println!("Setup completed successfully!"),
///     Err(e) => eprintln!("Setup failed: {}", e),
/// }
/// ```
pub fn gnu_linux_default_setup(allow_root: bool) -> Result<(), String> {
    let mut tasks = Vec::new();

    // Print license info
    print_license_info();

    tasks.push(TaskResult {
        status: 0,
        message: "License info displayed".to_string(),
    });

    // Validate root privileges
    let is_root = validate_root_priviliges(allow_root);

    tasks.push(TaskResult {
        status: if is_root || !allow_root { 0 } else { 1 },
        message: "Root privilege validation".to_string(),
    });

    // Set up user configuration
    let mut user_cfg = UserCfg::new();
    let user_name = match get_env_var("USER") {
        Ok(name) => name,
        Err(e) => return Err(e),
    };
    let home_dir = match get_env_var("HOME") {
        Ok(dir) => dir,
        Err(e) => return Err(e),
    };
    user_cfg.set_name(&user_name)?;
    user_cfg.set_home(&home_dir)?;
    println!("username: {}", user_cfg.get_name().green());
    println!("home location: {}", user_cfg.get_home().green());

    tasks.push(TaskResult {
        status: 0,
        message: "User configuration set".to_string(),
    });

    // Detect distribution
    let distro = detect_distro().unwrap_or_else(|| "unknown".to_string());

    // Set up iptables
    tasks.push(TaskResult {
        status: iptables_file_setup(),
        message: "iptables file setup".to_string(),
    });

    tasks.push(TaskResult {
        status: iptables_rules_setup(),
        message: "iptables rules setup".to_string(),
    });

    // Install software
    let sw_result = if check_sw_install_type() {
        let package_strings = set_sw_list();
        let package_slices: Vec<&str> = package_strings.iter().map(|s| s.as_str()).collect();
        software_setup(&package_slices, &distro)
    } else {
        software_setup(default_sw_package(), &distro)
    };

    tasks.push(TaskResult {
        status: sw_result,
        message: "Software installation".to_string(),
    });

    // Configure shells
    tasks.push(TaskResult {
        status: change_def_shell(user_cfg.get_name()),
        message: format!("Shell change for {}", user_cfg.get_name()),
    });

    tasks.push(TaskResult {
        status: change_def_shell("root"),
        message: "Shell change for root".to_string(),
    });

    // Set up Zsh
    tasks.push(TaskResult {
        status: install_omz(),
        message: "Oh My Zsh installation".to_string(),
    });

    tasks.push(TaskResult {
        status: install_zsh_autosuggestions(user_cfg.get_home()),
        message: "Zsh autosuggestions installation".to_string(),
    });

    tasks.push(TaskResult {
        status: install_zsh_syntax_highlighting(user_cfg.get_home()),
        message: "Zsh syntax highlighting installation".to_string(),
    });

    tasks.push(TaskResult {
        status: user_config_setup("../configs/.zshrc", user_cfg.get_home(), "zsh"),
        message: "Zsh user configuration".to_string(),
    });

    // Set up Vim
    tasks.push(TaskResult {
        status: user_config_setup("../configs/.vimrc", user_cfg.get_home(), "vim"),
        message: "Vim user configuration".to_string(),
    });

    // Configure root
    tasks.push(TaskResult {
        status: setup_root_config(user_cfg.get_home()),
        message: "Root configuration".to_string(),
    });

    // Set up ZRAM
    tasks.push(TaskResult {
        status: zram_swap_setup(),
        message: "ZRAM swap setup".to_string(),
    });

    // Validate all tasks
    if validate_task_statuses(tasks) {
        print_setup_status_success();
        Ok(())
    } else {
        Err("Setup failed. Check logs for details.".to_string())
    }
}

// Detects the Linux distribution by checking for specific release files.
//
// Returns `Some(String)` with the distribution name (e.g., "arch", "debian", "fedora")
// if detected, or `None` if the distribution is unknown. This function is used to
// tailor software installation commands to the detected distribution.
fn detect_distro() -> Option<String> {
    if std::path::Path::new("/etc/arch-release").exists() {
        Some("arch".to_string())
    } else if std::path::Path::new("/etc/debian_version").exists() {
        Some("debian".to_string())
    } else if std::path::Path::new("/etc/fedora-release").exists() {
        Some("fedora".to_string())
    } else {
        None
    }
}
