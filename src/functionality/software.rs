use colored::Colorize;
use std::process::Command;

/// installs software
pub fn software_setup(packages: &[String]) -> i8 {
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-Sy")
        .args(packages.iter().map(|s| s.as_str()))
        .arg("--noconfirm")
        .output()
        .expect("failed to install necessary software.");

    if output.status.success() {
        println!("software {}", "installed successfully".green());
        return 0;
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.contains("error:") && !stderr.contains("failed to download") {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
            eprintln!("{}", "warnings encountered during installation.".yellow());
            return 0;
        } else {
            eprintln!("{}{}", "error:\n".red(), stderr.red());
            return 1;
        }
    }
} // software_setup()
