use colored::Colorize;
use std::process::exit;

/// prints license information
pub fn print_license_info() {
    println!("gnulinwiz AKA GNU/Linux Config Wizard  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n");
} // print_license_info

/// validates if user has root priviliges. Terminates the program otherwise.
pub fn validate_root_priviliges() {
    if unsafe { libc::getuid() } != 0 {
        eprintln!("{}", "This program requires root privileges. Please run with sudo.\nExample: sudo ./gnulinwiz".red());
        // exits if not root
        exit(1);
    }
} // validate_root_priviliges

/// prints green-colored success message if set up successfully
pub fn print_setup_status_success() {
    println!(
        "{}",
        "All set! Your GNU/Linux system is ready to use!".green()
    );
} // print_setup_status_success

/// prints red-colored failure message if set up unseccessfully
pub fn print_setup_status_failed() {
    println!(
        "{}",
        "Something went wrong... Please fix the reported problems and re-run the program.".red()
    );
} // print_setup_status_failed
