use crate::functionality::prog_fun::print_setup_status_failed;
use colored::Colorize;
use std::process::{exit, Command};

/// validates task status
pub fn validate_task_status(status: i8) {
    if status != 0 {
        print_setup_status_failed();
        exit(status as i32);
    }
} // validate_task_status()

/// installs software
pub fn software_setup(packages: &[String]) -> i8 {
    let output = Command::new("pacman")
        .arg("-Sy")
        .args(packages.iter().map(|s| s.as_str()))
        .output()
        .expect("Failed to install necessary software.");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
        return 0;
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}{}", "Error:\n".red(), stderr.red());
        return 1;
    }
} // software_setup()

/// sets up username which is used in installation and configuration paths
fn username_setup() {
    todo!();
} // username_setup()

/// sets up iptables
fn iptables_setup() {
    todo!();
} // iptables_setup()

/// sets up zsh
fn zsh_setup() {
    todo!();
} // zsh_setup()

///sets up vim
fn vim_setup() {
    todo!();
} // vim_setup()

/// sets up configuration files for root user
fn root_setup() {
    todo!();
} // root_setup()

/// sets up zram swap configuration
fn zram_swap_setup() {
    todo!();
} // zram_swap_setup()

/// sets up zed code editor
fn zed_editor_setup() {
    todo!();
} // zed_editor_setup()
