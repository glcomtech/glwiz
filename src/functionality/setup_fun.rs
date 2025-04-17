use crate::functionality::prog_fun::print_setup_status_failed;
use colored::Colorize;
use std::{
    fs::{read_to_string, File},
    io::{Read, Write},
    path::Path,
    process::{exit, Command, Stdio},
};

/// validates task status
pub fn validate_task_status(status: i8) {
    if status != 0 {
        print_setup_status_failed();
        exit(status as i32);
    }
} // validate_task_status()

/// sets up iptables
pub fn iptables_file_setup() -> i8 {
    let source_path = Path::new("../configs/iptables.rules");
    let dest_path = Path::new("/etc/iptables/iptables.rules");

    match read_to_string(source_path) {
        Ok(rules) => match File::create(dest_path) {
            Ok(mut file) => {
                if file.write_all(rules.as_bytes()).is_ok() {
                    return 0;
                } else {
                    eprintln!("Error: Failed to write iptables rules to destination point.");
                    return 1;
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to create/open destination file: {}", e);
                return 2;
            }
        },
        Err(e) => {
            eprintln!(
                "Error: Failed to read iptables rules from source file: {}",
                e
            );
            return 3;
        }
    }
} // iptables_setup()

/// immediately sets up iptables rules
pub fn iptables_rules_setup() -> i8 {
    match File::open("/etc/iptables/iptables.rules") {
        Ok(mut file) => {
            let mut rules = String::new();
            if let Err(err) = file.read_to_string(&mut rules) {
                eprintln!(
                    "{}{}",
                    "Error reading rules file:\n".red(),
                    err.to_string().red()
                );
                return 1;
            }

            let mut command = Command::new("iptables-restore");
            command.stdin(Stdio::piped());

            match command.spawn() {
                Ok(mut child) => {
                    if let Some(mut stdin) = child.stdin.take() {
                        if let Err(err) = std::io::Write::write_all(&mut stdin, rules.as_bytes()) {
                            eprintln!(
                                "{}{}",
                                "Error writing to stdin:\n".red(),
                                err.to_string().red()
                            );
                            return 1;
                        }
                    }

                    let output = child
                        .wait_with_output()
                        .expect("Failed to wait for command.");

                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("{}", stdout);
                        return 0;
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!(
                            "{}{}",
                            "Error applying iptables rules:\n".red(),
                            stderr.red()
                        );
                        return 1;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{}{}",
                        "Error spawning iptables-restore:\n".red(),
                        e.to_string().red()
                    );
                    return 1;
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}",
                "Error opening rules file:\n".red(),
                e.to_string().red()
            );
            return 1;
        }
    }
} // iptables_rules_setup()

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
