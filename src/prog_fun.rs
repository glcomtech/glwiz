use colored::Colorize;
use std::{io::stdin, process::exit};

/// default software installation package
pub fn default_package() -> Vec<String> {
    vec![
        "firefox".to_string(),
        "clang".to_string(),
        "zsh".to_string(),
        "git".to_string(),
        "zed".to_string(),
        "gimp".to_string(),
        "mpv".to_string(),
        "spectacle".to_string(),
    ]
} // default_package()

/// prints license information
pub fn print_license_info() {
    println!("gnulinwiz AKA GNU/Linux Config Wizard  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n");
} // print_license_info()

/// validates if user has root priviliges. Terminates the program otherwise.
pub fn validate_root_priviliges() {
    if unsafe { libc::getuid() } != 0 {
        eprintln!("{}", "This program requires root privileges. Please run with sudo.\nExample: sudo ./gnulinwiz".red());
        // exits if not root
        exit(1);
    }
} // validate_root_priviliges()

/// prints green-colored success message if set up successfully
pub fn print_setup_status_success() {
    println!(
        "{}",
        "All set! Your GNU/Linux system is ready to use!".green()
    );
} // print_setup_status_success()

/// prints red-colored failure message if set up unseccessfully
pub fn print_setup_status_failed() {
    println!(
        "{}",
        "Something went wrong... Please fix the reported problems and re-run the program.".red()
    );
} // print_setup_status_failed()

/// checks wether user wants to use default installation list or to enter a custom one
pub fn check_sw_install_type() -> bool {
    println!("{}", "Enter 0 for the default list of software or 1 to enter a custom list:".yellow());
    
    let input = read_input().trim().to_string();
    
    match input.parse::<i8>() {
        Ok(value) if value == 1 => {
            println!("{}", "You chose to enter a custom list.".green());
            true
        }
        Ok(_) => {
            println!("{}", "You chose to use the default list.".green());
            false
        }
        Err(_) => {
            handle_error("Please enter a valid number (0 or 1).");
            false // Return false in case of error, or you can choose to exit
        }
    }
} // check_sw_install_type

/// Reads a line of input from the user.
fn read_input() -> String {
    let mut input = String::new();
    if let Err(error) = stdin().read_line(&mut input) {
        handle_error(&format!("{}{}", "Error reading input:".red(), error));
        exit(1);
    }
    input
} // read_input

/// Handles errors by printing the error message and performing any necessary cleanup.
fn handle_error(message: &str) {
    eprintln!("{}{}", "Error: ".red(), message);
    print_setup_status_failed();
    exit(1);
} // handle_error

/// sets a list of software to install
pub fn set_sw_list() -> Vec<String> {
    let mut packages = Vec::new();
    let mut input = String::new();

    println!("Enter the software packages to install (separated by spaces):");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    packages.extend(input.trim().split_whitespace().map(String::from));

    return packages;
} // set_software()
