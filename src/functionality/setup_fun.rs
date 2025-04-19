use crate::functionality::prog_fun::print_setup_status_failed;
use colored::Colorize;
use std::{
    env::var,
    fs::read_to_string,
    io::Write,
    path::Path,
    process::Stdio,
    process::{exit, Command},
};

/// gets environment variables
fn get_env_var(env_var: &str) -> Option<String> {
    match var(env_var) {
        Ok(username) => Some(username),
        Err(_) => {
            eprintln!(
                "{}\n",
                "error: could not determine the environment variables.".red()
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

/// runs sudo commands
fn run_sudo_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "failed to execute command:".red(), command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
} // run_sudo_command

/// runs user commands
fn run_user_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("{} {}: {}", "failed to execute command:".red(), command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command `{}` failed:\nstdout: {}\nstderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
} // run_sudo_command

/// runs sudo commands with stdin
fn run_sudo_command_with_stdin(
    command: &str,
    args: &[&str],
    stdin_content: String,
) -> Result<(), String> {
    let mut cmd = Command::new("sudo")
        .arg(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to spawn command `{}`: {}", command, e))?;

    if let Some(mut stdin) = cmd.stdin.take() {
        stdin
            .write_all(stdin_content.as_bytes())
            .map_err(|e| format!("failed to write to stdin of `{}`: {}", command, e))?;
    }

    let output = cmd
        .wait_with_output()
        .map_err(|e| format!("failed to wait for command `{}`: {}", command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command `{}` failed:\nStdout: {}\nStderr: {}",
            command.red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// sets up iptables
pub fn iptables_file_setup() -> i8 {
    let source_path = Path::new("../configs/iptables.rules");
    let dest_path = Path::new("/etc/iptables/iptables.rules");

    match read_to_string(source_path) {
        Ok(rules) => {
            let command = "tee";
            let args = &[dest_path.as_os_str().to_str().unwrap()];

            match run_sudo_command_with_stdin(command, args, rules) {
                Ok(_) => {
                    println!("iptables.rules {}", "created successfully".green());
                    return 0;
                }
                Err(e) => {
                    eprintln!("error setting up iptables file: {}", e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("failed to read iptables rules from source file: {}", e);
            2
        }
    }
} // iptables_setup()

/// immediately sets up iptables rules
pub fn iptables_rules_setup() -> i8 {
    let rules_path = String::from("/etc/iptables/iptables.rules");
    let command = "bash";
    let args = &["-c", &format!("iptables-restore < {}", rules_path)];

    match run_sudo_command(command, args) {
        Ok(_) => {
            println!("iptables.rules {}", "set successfully".green());
            0
        }
        Err(e) => {
            eprintln!("error applying iptables rules: {}", e);
            1
        }
    }
} // iptables_rules_setup()

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

/// installs oh my zsh
pub fn install_omz() -> i8 {
    let command = "curl";
    let args = &[
        "-fsSL",
        "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh",
    ];

    match run_user_command(command, args) {
        Ok(_) => {
            println!("oh-my-zsh {}", "installed successfully".green());
            return 0;
        }
        Err(e) => {
            eprintln!("{}{}", "error downloading oh-my-zsh:".red(), e.red());
            return 1;
        }
    }
} // install_omz

/// installs zsh autosuggestions plugin
pub fn install_zsh_autosuggestions(home_dir: String) -> i8 {
    let command = "git";
    let zsh_custom_path = format!("{}/.oh-my-zsh/custom/plugins/zsh-autosuggestions", home_dir);
    let args = &[
        "clone",
        "https://github.com/zsh-users/zsh-autosuggestions",
        &zsh_custom_path,
    ];

    match run_user_command(command, args) {
        Ok(_) => {
            println!("zsh-autosuggestions {}", "installed successfully".green());
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{}{}",
                "error downloading zsh-autosuggestions:".red(),
                e.red()
            );
            return 1;
        }
    }
} // install_zsh_autosuggestions

/// installs zsh syntax highlighting plugin
pub fn install_zsh_syntax_highlighting(home_dir: String) -> i8 {
    let command = "git";
    let zsh_custom_path = format!(
        "{}/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting",
        home_dir
    );
    let args = &[
        "clone",
        "https://github.com/zsh-users/zsh-syntax-highlighting.git",
        &zsh_custom_path,
    ];

    match run_user_command(command, args) {
        Ok(_) => {
            println!(
                "zsh-syntax-highlighting {}",
                "installed successfully".green()
            );
            return 0;
        }
        Err(e) => {
            eprintln!(
                "{}{}",
                "error downloading zsh-syntax-highlighting:".red(),
                e.red()
            );
            return 1;
        }
    }
} // install_zsh_syntax_highlighting

/// sets up config files in home directory
pub fn user_config_setup(config_path: String, home_dir: String, cfg_name: &str) -> i8 {
    let source = Path::new(&config_path);
    let filename = source.file_name();

    match filename {
        Some(name) => {
            let destination_path = Path::new(&home_dir).join(name);
            match std::fs::copy(config_path, &destination_path) {
                Ok(_) => {
                    println!("{} {}", cfg_name, "custom config was installed".green());
                    return 0;
                }
                Err(e) => {
                    eprintln!(
                        "error: custom config failed to install {} to '{}': {}",
                        cfg_name,
                        destination_path.display(),
                        e
                    );
                    return 1;
                }
            }
        }
        None => {
            eprintln!(
                "error: could not determine filename from path: {}",
                config_path.red()
            );
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
        Ok(_) => println!("/root/.oh-my-zsh {}", "created symbolic link".green()),
        Err(e) => {
            eprintln!(
                "{}{}",
                "error creating symbolic link for /root/.oh-my-zsh:".red(),
                e.red()
            );
            return 1;
        }
    }

    // Create symbolic link for .zshrc
    match run_sudo_command("ln", &["-s", zshrc_src.as_str(), zshrc_dest.as_str()]) {
        Ok(_) => println!("/root/.zshrc {}", "created symbolic link for ".green()),
        Err(e) => {
            eprintln!(
                "{}{}",
                "error creating symbolic link for /root/.zshrc:".red(),
                e.red()
            );
            return 2;
        }
    }

    // Create symbolic link for .vimrc
    match run_sudo_command("ln", &["-s", vimrc_src.as_str(), vimrc_dest.as_str()]) {
        Ok(_) => println!("/root/.vimrc {}", "created symbolic link".green()),
        Err(e) => {
            eprintln!(
                "{}{}",
                "error creating symbolic link for /root/.vimrc:".red(),
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
                println!("zram {}", "swap configuration copied successfully.".green());
                return 0;
            } else {
                eprintln!(
                    "{}{}",
                    "error copying zram-generator.conf:".red(),
                    String::from_utf8_lossy(&output.stderr).red()
                );
                return 1;
            }
        }
        Err(e) => {
            eprintln!(
                "{}{}",
                "error executing command:".red(),
                e.to_string().red()
            );
            return 2;
        }
    }
} // zram_swap_setup()
