/*
 * GLWiz - The ultimate post-installation setup assistant for GNU/Linux popular distros,
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

use colored::Colorize;
use std::io::{self, Write};
use std::process::exit;

/// Returns a default list of software packages for installation.
///
/// This function provides a predefined list of commonly used software packages (e.g., Firefox, Vim)
/// for the "glwiz" project’s post-installation setup. It is used when the user opts for the default
/// software installation instead of a custom list. The returned slice is static to ensure consistent
/// access across the setup process.
///
/// # Returns
/// A static slice of strings containing package names (e.g., `["firefox", "clang", ...]`).
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::default_sw_package;
/// let packages = default_sw_package();
/// assert_eq!(packages, &["firefox", "clang", "zsh", "git", "gimp", "mpv", "curl", "vim", "rkhunter"]);
/// ```
///
/// # See Also
/// - `software::software_setup`: Uses this list for default software installation.
/// - `check_sw_install_type`: Determines whether to use this list or a custom one.
pub fn default_sw_package() -> &'static [&'static str] {
    &[
        "firefox", "clang", "zsh", "git", "gimp", "mpv", "curl", "vim", "rkhunter",
    ]
}

/// Displays the GNU/Linux Config Wizard’s license information.
///
/// This function prints the GNU General Public License (GPL) v3 notice for the "glwiz" project,
/// including a link to the full license text. It is called at the start of the setup process to inform
/// users of the software’s licensing terms and warranty disclaimer.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::print_license_info;
/// print_license_info(); // Outputs GPL v3 notice with license link
/// ```
///
/// # See Also
/// - `gnu_linux_default_setup`: Calls this function as part of the setup process.
pub fn print_license_info() {
    let link = "https://www.gnu.org/licenses/gpl-3.0.html".blue();
    println!(
        "GLWiz Copyright (C) 2025  Andrew Kushyk\n\
 This program comes with ABSOLUTELY NO WARRANTY; for details see {}\n\
 This is free software, and you are welcome to redistribute it\n\
 under certain conditions; for details see {}\n",
        link, link
    );
}

/// Validates whether the program is running with root privileges.
///
/// This function checks if the program is executed as the root user by inspecting the user ID (UID).
/// In the "glwiz" project, it ensures safe execution by enforcing root privilege policies. If run
/// as root without explicit permission, it terminates the program with an error message. Otherwise,
/// it logs the privilege status and returns the result.
///
/// # Arguments
/// * `allow_root` - If `true`, permits execution as root; if `false`, terminates if run as root.
///
/// # Returns
/// * `true` - The program is running as root and `allow_root` is `true`.
/// * `false` - The program is not running as root.
///
/// # Safety
/// This function uses `unsafe` to call `libc::getuid` for UID checking. It is safe as long as the
/// underlying system call behaves as expected on Unix-like systems.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::validate_root_priviliges;
/// let is_root = validate_root_priviliges(true);
/// if is_root {
///     println!("Running as root");
/// } else {
///     println!("Running as non-root");
/// }
/// ```
///
/// # See Also
/// - `gnu_linux_default_setup`: Uses this function to validate privileges during setup.
pub fn validate_root_priviliges(allow_root: bool) -> bool {
    if unsafe { libc::getuid() } == 0 {
        if !allow_root {
            eprintln!(
                "{}",
                "Running as root is not recommended. Use --allow-root to proceed.".red()
            );
            exit(1);
        }
        println!("{}", "Running with root privileges.".green());
        true
    } else {
        false
    }
}

/// Prints a success message indicating a completed setup.
///
/// This function displays a confirmation message when the "glwiz" setup process completes
/// successfully, informing the user that their GNU/Linux system is ready. It uses colored output
/// for better visibility and is called after all configuration tasks are validated.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::print_setup_status_success;
/// print_setup_status_success(); // Outputs "All set! Your GNU/Linux system is ready to use!"
/// ```
///
/// # See Also
/// - `task::validate_task_statuses`: Determines when to call this function.
/// - `print_setup_status_failed`: The counterpart for failed setups.
pub fn print_setup_status_success() {
    println!(
        "{}",
        "All set! Your GNU/Linux system is ready to use!".green()
    );
}

/// Prints a failure message for setup errors.
///
/// This function displays an error message when the "glwiz" setup process fails, prompting the
/// user to check error logs. It uses colored output for emphasis and is called when configuration
/// tasks do not complete successfully.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::print_setup_status_failed;
/// print_setup_status_failed(); // Outputs "Setup failed. Please check error messages and try again."
/// ```
///
/// # See Also
/// - `task::validate_task_statuses`: Determines when to call this function.
/// - `print_setup_status_success`: The counterpart for successful setups.
pub fn print_setup_status_failed() {
    println!(
        "{}",
        "Setup failed. Please check error messages and try again.".red()
    );
}

/// Prompts the user to choose between default or custom software lists.
///
/// This function interactively asks the user to select a software installation mode in the
/// "glwiz" project. Entering `0` selects a custom list, while any other number selects the
/// default list. It loops until valid input is provided, ensuring robust user interaction.
///
/// # Returns
/// * `true` - The user selected a custom software list.
/// * `false` - The user selected the default software list.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::check_sw_install_type;
/// let use_custom = check_sw_install_type();
/// if use_custom {
///     println!("User chose custom software list");
/// } else {
///     println!("User chose default software list");
/// }
/// ```
///
/// # See Also
/// - `read_input`: Used to capture user input.
/// - `default_sw_package`: Provides the default list if selected.
/// - `set_sw_list`: Collects the custom list if selected.
pub fn check_sw_install_type() -> bool {
    loop {
        println!(
            "{}",
            "Enter 0 for a custom software list or any other number for default:".yellow()
        );

        let input = read_input();
        match input.trim().parse::<i8>() {
            Ok(0) => {
                println!("{}", "Selected custom software list.".green());
                return true;
            }
            Ok(_) => {
                println!("{}", "Selected default software list.".green());
                return false;
            }
            Err(_) => println!("{}", "Invalid input. Please enter a number.".red()),
        }
    }
}

/// Reads a line of input from standard input (stdin).
///
/// This function captures a single line of user input in the "glwiz" project, used for
/// interactive tasks like prompting for software lists or overwrite confirmations. It flushes
/// stdout to ensure prompts are displayed and expects input to be valid UTF-8, panicking on
/// I/O errors for simplicity.
///
/// # Returns
/// A `String` containing the user’s input, including the trailing newline.
///
/// # Panics
/// Panics if reading from stdin fails (e.g., due to I/O errors). This is intentional for simplicity,
/// as stdin is expected to be available in an interactive context.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::read_input;
/// let input = read_input();
/// println!("User entered: {}", input.trim());
/// ```
///
/// # See Also
/// - `check_sw_install_type`: Uses this function for user input.
/// - `set_sw_list`: Uses this function to collect custom package names.
pub fn read_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

/// Collects custom software packages from user input.
///
/// This function prompts the user to enter a space-separated list of software packages for
/// installation in the "glwiz" project. It splits the input into individual package names
/// and returns them as a vector of strings, used when the user selects a custom installation mode.
///
/// # Returns
/// A `Vec<String>` containing the user-specified package names.
///
/// # Example
/// ```
/// use glwiz::functionality::prog_fun::set_sw_list;
/// let packages = set_sw_list();
/// println!("Custom packages: {:?}", packages);
/// ```
///
/// # See Also
/// - `read_input`: Used to capture the user’s package list.
/// - `check_sw_install_type`: Determines when to call this function.
/// - `software::software_setup`: Installs the collected packages.
pub fn set_sw_list() -> Vec<String> {
    println!("Enter software packages to install (space-separated):");
    let input = read_input();
    input.trim().split_whitespace().map(String::from).collect()
}
