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

/// Provides utilities for executing system commands, including `sudo` and user-level operations.
///
/// This submodule contains functions to run commands with elevated privileges or as the current user,
/// handling tasks like copying files or executing shell scripts. It is used across other submodules
/// for operations requiring system-level access.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::commands::run_sudo_command;
/// let result = run_sudo_command("echo", &["Hello, world!"]);
/// assert!(result.is_ok());
/// ```
pub mod commands;

/// Manages user and root configuration file setups, such as `.zshrc` and `.vimrc`.
///
/// This submodule handles copying configuration files to user and root directories, ensuring
/// consistent environments. It supports idempotent operations by checking for existing files
/// and prompting for overwrites.
///
/// # See Also
/// - `user_cfg`: For managing user-specific configuration data.
pub mod configs;

/// Retrieves environment variables for system configuration.
///
/// This submodule provides functions to access environment variables like `USER` and `HOME`,
/// which are critical for user-specific setups. It returns errors for unset variables to ensure
/// robust error handling.
pub mod env;

/// Configures iptables firewall rules for network security.
///
/// This submodule sets up and applies iptables rules by copying configuration files and
/// executing restore commands. It ensures existing rules are preserved or overwritten with
/// user confirmation.
pub mod iptables;

/// Contains utility functions for common setup tasks, such as user input handling and license display.
///
/// This submodule provides functions for tasks like prompting users for software lists, validating
/// root privileges, and displaying license information. It is used to enhance user interaction
/// and setup logic.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::prog_fun::print_license_info;
/// print_license_info(); // Displays the GNU GPL v3 license information
/// ```
pub mod prog_fun;

/// Manages shell configurations, including Zsh setup and plugin installations.
///
/// This submodule handles changing the default shell to Zsh, installing Oh My Zsh, and adding
/// plugins like autosuggestions and syntax highlighting. It ensures idempotent operations to
/// avoid redundant setups.
///
/// # See Also
/// - `configs`: For related configuration file management.
pub mod shell;

/// Handles software package installation across supported Linux distributions.
///
/// This submodule detects the Linux distribution and uses the appropriate package manager
/// (e.g., `pacman`, `apt`, `dnf`) to install software. It supports both default and custom
/// package lists.
pub mod software;

/// Tracks and validates the results of configuration tasks.
///
/// This submodule defines the `TaskResult` struct and functions to validate task outcomes,
/// ensuring comprehensive error reporting for the setup process. It is used to collect and
/// summarize task statuses.
pub mod task;

// Stores and manages user configuration data, such as username and home directory.
///
/// This submodule provides the `UserCfg` struct to hold user-specific data, with methods to
/// set and retrieve usernames and home directories. It is used for user-specific configurations
/// like shell and Vim setups.
///
/// # See Also
/// - `configs`: For applying user configuration files.
pub mod user_cfg;

/// Configures ZRAM swap for improved system performance.
///
/// This submodule sets up compressed RAM-based swap by copying configuration files to the
/// appropriate system directories. It checks for existing configurations to avoid overwrites
/// unless confirmed by the user.
///
/// # Example
/// ```should_panic
/// // Requires ../configs/zram-generator.conf and sudo privileges.
/// // Use integration tests for actual validation.
/// use gnulinwiz::functionality::zram::zram_swap_setup;
/// let result = zram_swap_setup();
/// assert_eq!(result, 0); // Success if config exists and no overwrite
/// ```
pub mod zram;
