use crate::functionality::prog_fun::print_setup_status_failed;
use colored::Colorize;
use std::{
    env::var,
    fs::{read_to_string, File},
    io::{Read, Write},
    path::Path,
    process::{exit, Command, Stdio},
};

/// gets environment variables
fn get_env_var(env_var: &str) -> Option<String> {
    match var(env_var) {
        Ok(username) => Some(username),
        Err(_) => {
            eprintln!(
                "{}\n",
                "ERROR: Could not determine the environment variables.".red()
            );
            None
        }
    }
} // get_env_var

/// validates environment variables
pub fn validate_env_var(env_var: &str) -> String {
    match get_env_var(env_var) {
        Some(env_var) => env_var,
        None => "".to_string(),
    }
}

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
                    eprintln!(
                        "{}\n",
                        "Error: Failed to write iptables rules to destination point.".red()
                    );
                    return 1;
                }
            }
            Err(e) => {
                eprintln!(
                    "{} {}\n",
                    "Error: Failed to create/open destination file:".red(),
                    e
                );
                return 2;
            }
        },
        Err(e) => {
            eprintln!(
                "{} {}\n",
                "Error: Failed to read iptables rules from source file:".red(),
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
                    "{}{}\n",
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
                                "{}{}\n",
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
                            "{}{}\n",
                            "Error applying iptables rules:\n".red(),
                            stderr.red()
                        );
                        return 1;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{}{}\n",
                        "Error spawning iptables-restore:\n".red(),
                        e.to_string().red()
                    );
                    return 1;
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}\n",
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
        eprintln!("{}{}\n", "Error:\n".red(), stderr.red());
        return 1;
    }
} // software_setup()

/// runs sudo commands
fn run_sudo_command(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new("sudo")
        .arg(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "Failed to execute command:".red(), command, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(format!(
            "Command `{}` failed:\nStdout: {}\nStderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
} // run_sudo_command

/// runs commands as user
fn run_command_as(command: &str, args: &[&str], user: &str) -> i8 {
    let su_command = "su";
    let su_arg = format!(
        "-c '{}'",
        format!("zsh -c \"{}\"", format!("{} {}", command, args.join(" ")))
    );

    let output_result = Command::new(su_command).arg(user).arg(su_arg).output();

    match output_result {
        Ok(output) => {
            if output.status.success() {
                return 0;
            } else {
                eprintln!(
                    "Command `{}` as user `{}` failed:\nStdout: {}\nStderr: {}",
                    command.red(),
                    user.red(),
                    String::from_utf8_lossy(&output.stdout).trim(),
                    String::from_utf8_lossy(&output.stderr).trim()
                );
                return 1; // Or some other non-zero error code
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to execute command `{}` as user `{}`: {}",
                command.red(),
                user.red(),
                e
            );
            return 2; // Or a different non-zero error code to distinguish execution failure
        }
    }
} // run_command_as

/// installs oh my zsh
pub fn install_omz(username: String) -> i8 {
    let command = "curl";
    let args = &[
        "-fsSL",
        "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh",
    ];
    run_command_as(command, args, username.as_str())
} // install_omz

/// installs zsh autosuggestions plugin
pub fn install_zsh_autosuggestions(username: String, home_dir: String) -> i8 {
    let command = "git";
    let zsh_custom_path = format!("{}/.oh-my-zsh/custom/plugins", home_dir);
    let args = &[
        "clone",
        "https://github.com/zsh-users/zsh-autosuggestions",
        &zsh_custom_path,
    ];
    run_command_as(command, args, username.as_str())
} // install_zsh_autosuggestions

/// installs zsh syntax highlighting plugin
pub fn install_zsh_syntax_highlighting(username: String, home_dir: String) -> i8 {
    let command = "git";
    let zsh_custom_path = format!("{}/.oh-my-zsh/custom/plugins", home_dir);
    let args = &[
        "clone",
        "https://github.com/zsh-users/zsh-syntax-highlighting.git",
        &zsh_custom_path,
    ];
    run_command_as(command, args, username.as_str())
} // install_zsh_syntax_highlighting

/// sets up config files in home directory
pub fn user_config_setup(config_path: String, home_dir: String) -> i8 {
    let destination_path = String::from(format!("{}/", home_dir));

    match std::fs::copy(config_path, &destination_path) {
        Ok(_) => {
            return 0;
        }
        Err(_) => {
            return 1;
        }
    }
} // user_config_setup

/// sets up root config in /root directory
pub fn setup_root_config(home_dir: String) -> i8 {
    let oh_my_zsh_src = format!("{}{}", home_dir, "/.oh-my-zsh");
    let oh_my_zsh_dest = String::from("/root/.oh-my-zsh");
    let zshrc_src = format!("{}{}", home_dir, "/.zshrc");
    let zshrc_dest = String::from("/root/.zshrc");
    let vimrc_src = format!("{}{}", home_dir, "/.vimrc");
    let vimrc_dest = String::from("/root/.vimrc");

    // Create symbolic link for .oh-my-zsh
    match run_sudo_command(
        "ln",
        &["-s", oh_my_zsh_src.as_str(), oh_my_zsh_dest.as_str()],
    ) {
        Ok(_) => println!(
            "{}\n",
            "Created symbolic link for /root/.oh-my-zsh\n".green()
        ),
        Err(e) => {
            eprintln!(
                "{}{}\n",
                "Error creating symbolic link for /root/.oh-my-zsh:\n".red(),
                e.red()
            );
            return 1;
        }
    }

    // Create symbolic link for .zshrc
    match run_sudo_command("ln", &["-s", zshrc_src.as_str(), zshrc_dest.as_str()]) {
        Ok(_) => println!("{}\n", "Created symbolic link for /root/.zshrc\n".green()),
        Err(e) => {
            eprintln!(
                "{}{}\n",
                "Error creating symbolic link for /root/.zshrc:\n".red(),
                e.red()
            );
            return 2;
        }
    }

    // Create symbolic link for .vimrc
    match run_sudo_command("ln", &["-s", vimrc_src.as_str(), vimrc_dest.as_str()]) {
        Ok(_) => println!("{}\n", "Created symbolic link for /root/.vimrc\n".green()),
        Err(e) => {
            eprintln!(
                "{}{}\n",
                "Error creating symbolic link for /root/.vimrc:\n".red(),
                e.red()
            );
            return 3;
        }
    }

    return 0;
} // setup_root_config

/// sets up zram swap configuration
pub fn zram_swap_setup() -> i8 {
    let output = Command::new("sudo")
        .arg("cp")
        .arg("../configs/zram-generator.conf")
        .arg("/etc/systemd/")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "{}\n",
                    "ZRAM swap configuration copied successfully.".green()
                );
                return 0;
            } else {
                eprintln!(
                    "{}{}\n",
                    "Error copying zram-generator.conf:\n".red(),
                    String::from_utf8_lossy(&output.stderr).red()
                );
                return 1;
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}\n",
                "Error executing command:\n".red(),
                e.to_string().red()
            );
            return 2;
        }
    }
} // zram_swap_setup()
