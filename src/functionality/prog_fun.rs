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

/// default software installation package list
/// Returns a slice of static strings.
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

/// prints license information
pub fn print_license_info() {
    let link = "https://www.gnu.org/licenses/gpl-3.0.html/".blue();
    println!(
        "gnulinwiz AKA GNU/Linux Config Wizard  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see {}\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see {}\n",
        link, link
    );
}

/// validates if user has root privileges. Terminates the program otherwise.
///
/// Running this tool with root privileges is generally unsafe and unnecessary.
/// This check prevents root execution.
pub fn validate_root_priviliges() {
    if unsafe { libc::getuid() } == 0 {
        eprintln!("{}", "this program is not recommended to run with root privileges. please run it with your current user.\nexample: ./gnulinwiz".red());
        // exits if root
        exit(1);
    }
}

/// prints green-colored success message if set up successfully
pub fn print_setup_status_success() {
    println!(
        "{}",
        "all set! your gnu/linux system is ready to use!".green()
    );
}

/// prints red-colored failure message if set up unseccessfully
pub fn print_setup_status_failed() {
    println!(
        "{}",
        "something went wrong... please fix the reported problems and re-run the program.".red()
    );
}

/// checks whether user wants to use default installation list or to enter a custom one
/// Returns true for custom list, false for default. Exits on invalid input.
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

/// Reads a line of input from the user.
/// Handles read errors by calling handle_error and exiting.
fn read_input() -> String {
    let mut input = String::new();
    if let Err(error) = stdin().read_line(&mut input) {
        handle_error(&format!("{}{}", "error reading input: ".red(), error))
    }
    return input;
}

/// Handles errors by printing the error message and exiting.
pub fn handle_error(message: &str) -> ! {
    eprintln!("{}{}", "error: ".red(), message);
    print_setup_status_failed();
    exit(1);
}

/// sets a list of software to install by reading user input
/// Uses the read_input function and handles potential errors via it.
pub fn set_sw_list() -> Vec<String> {
    println!("enter the software packages to install (separated by spaces):");
    let input = read_input();
    return input.trim().split_whitespace().map(String::from).collect();
}
