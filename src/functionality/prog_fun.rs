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
use std::{io::stdin, process::exit};

/// Returns the default list of software packages for installation.
///
/// Provides a static list of commonly used software packages to be installed
/// during the GNU/Linux configuration process.
///
/// # Returns
/// A static slice of package names as strings (e.g., "firefox", "zsh").
///
/// # Examples
/// ```
/// let packages = default_sw_package();
/// assert_eq!(packages, &["firefox", "clang", "zsh", "git", "gimp", "mpv", "spectacle", "curl"]);
/// ```
pub fn default_sw_package() -> &'static [&'static str] {
    &[
        "firefox",
        "clang",
        "zsh",
        "git",
        "gimp",
        "mpv",
        "spectacle",
        "curl",
    ]
}

/// Prints the GNU/Linux Config Wizard's license information to the console.
///
/// Displays the copyright notice, warranty disclaimer, and redistribution terms,
/// including a link to the GNU General Public License (GPL) v3.
pub fn print_license_info() {
    let link = "https://www.gnu.org/licenses/gpl-3.0.html".blue();
    println!(
        "gnulinwiz AKA GNU/Linux Config Wizard  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see {}\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see {}\n",
        link, link
    );
}

/// Validates that the program is not running with root privileges.
///
/// Checks the user ID to ensure the program is not executed as root, as running
/// with root privileges is considered unsafe and unnecessary. If root privileges
/// are detected, an error message is printed, and the program exits with status code 1.
///
/// # Panics
/// Exits the program with status code 1 if the user ID is 0 (root).
pub fn validate_root_priviliges() {
    if unsafe { libc::getuid() } == 0 {
        eprintln!("{}", "this program is not recommended to run with root privileges. please run it with your current user.\nexample: ./gnulinwiz".red());
        // exits if root
        exit(1);
    }
}

/// Prints a green-colored success message to indicate a successful setup.
///
/// Displays a confirmation message to the user when the GNU/Linux configuration
/// process completes without errors.
pub fn print_setup_status_success() {
    println!(
        "{}",
        "all set! your gnu/linux system is ready to use!".green()
    );
}

/// Prints a red-colored failure message to indicate an unsuccessful setup.
///
/// Displays a message prompting the user to fix reported issues and re-run the program
/// when the configuration process encounters an error.
pub fn print_setup_status_failed() {
    println!(
        "{}",
        "something went wrong... please fix the reported problems and re-run the program.".red()
    );
}

/// Prompts the user to choose between default or custom software installation lists.
///
/// Asks the user to input a number: 0 for a custom list or any other number for the default list.
/// Returns `true` for a custom list and `false` for the default list. Invalid input triggers
/// an error and program exit via `handle_error`.
///
/// # Returns
/// * `true` if the user selects a custom list (input is 0).
/// * `false` if the user selects the default list (input is any non-zero number).
///
/// # Panics
/// Exits the program via `handle_error` if the input cannot be parsed as an integer.
///
/// # Examples
/// ```
/// // Simulating user input of "0" would return true
/// // Simulating user input of "1" would return false
/// let use_custom = check_sw_install_type();
/// ```
pub fn check_sw_install_type() -> bool {
    println!(
        "{}",
        "enter any number for the default list of software or 0 to enter a custom list:".yellow()
    );

    let input = read_input();

    match input.trim().parse::<i8>() {
        Ok(value) if value == 0 => {
            println!("{}", "you chose to enter a custom list.\ninstallation takes a few minutes, please wait...".green());
            return true;
        }
        Ok(_) => {
            println!("{}", "you chose to use the default list.\ninstallation takes a few minutes, please wait...".green());
            return false;
        }
        Err(_) => handle_error(
            "please enter 0 for a custom list or any other number for the default list.",
        ),
    }
}

// Reads a line of input from stdin and returns it as a String.
// Handles read errors by calling handle_error and exiting the program.
fn read_input() -> String {
    let mut input = String::new();
    if let Err(error) = stdin().read_line(&mut input) {
        handle_error(&format!("{}{}", "error reading input: ".red(), error))
    }
    return input;
}

/// Handles errors by printing a message and exiting the program.
///
/// Prints the provided error message in red, followed by a call to `print_setup_status_failed`,
/// and terminates the program with exit code 1.
///
/// # Arguments
/// * `message` - The error message to display.
///
/// # Panics
/// Always exits the program with status code 1 after printing the error message.
pub fn handle_error(message: &str) -> ! {
    eprintln!("{}{}", "error: ".red(), message);
    print_setup_status_failed();
    exit(1);
}

/// Collects a list of software packages from user input for installation.
///
/// Prompts the user to enter a space-separated list of software package names,
/// reads the input, and splits it into a vector of strings. Input errors are handled
/// via the `read_input` function.
///
/// # Returns
/// A vector of owned `String`s containing the package names entered by the user.
///
/// # Examples
/// ```
/// // Simulating user input of "vim nano" would return vec!["vim".to_string(), "nano".to_string()]
/// let packages = set_sw_list();
/// ```
pub fn set_sw_list() -> Vec<String> {
    println!("enter the software packages to install (separated by spaces):");
    let input = read_input();
    return input.trim().split_whitespace().map(String::from).collect();
}
