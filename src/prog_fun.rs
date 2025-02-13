use colored::Colorize;

/// prints license information
pub fn print_license_info() {
    println!("gnulinwiz AKA GNU/Linux Config Wizard  Copyright (C) 2025  Andrew Kushyk\n\
This program comes with ABSOLUTELY NO WARRANTY; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n\
This is free software, and you are welcome to redistribute it\n\
under certain conditions; for details see https://www.gnu.org/licenses/gpl-3.0.html/\n");
} // print_license_info

/// prints green-colored success message if set up successfully
pub fn print_setup_status_success() {
    println!(
        "{}",
        "All set! Your GNU/Linux system is ready to use!".green()
    );
} // print_setup_status_success

pub fn print_setup_status_failed() {
    println!(
        "{}",
        "Something went wrong... Please fix the reported problems and re-run the program.".red()
    );
} // print_setup_status_failed
