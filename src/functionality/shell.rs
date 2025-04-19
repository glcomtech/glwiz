use super::commands::{run_sudo_command, run_user_command};
use colored::Colorize;
use std::process::{Command, Stdio};

/// changes default shell to zsh
pub fn change_def_shell(name: String) -> i8 {
    let command = "chsh";
    let args = &["-s", "/usr/bin/zsh", name.as_str()];

    match run_sudo_command(command, args) {
        Ok(_) => {
            println!("zsh {}", "shell set successfully".green());
            0
        }
        Err(e) => {
            eprintln!("zsh {} {}", "is already default shell".red(), e);
            1
        }
    }
} // change_def_shell

/// installs oh my zsh
pub fn install_omz() -> i8 {
    let mut curl_cmd = match Command::new("curl")
        .args(&[
            "-fsSL",
            "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh",
        ])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{} {}", "error starting curl command:".red(), e);
            return 1;
        }
    };

    let curl_stdout = match curl_cmd.stdout.take() {
        Some(stdout) => stdout,
        None => {
            eprintln!("{}", "error: could not capture curl stdout".red());
            let _ = curl_cmd.wait();
            return 1;
        }
    };

    let mut bash_cmd = match Command::new("bash")
        .stdin(Stdio::from(curl_stdout))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!(
                "{} {}",
                "error starting bash command for script execution:".red(),
                e
            );
            let _ = curl_cmd.wait();
            return 1;
        }
    };

    let curl_status = match curl_cmd.wait() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("{} {}", "error waiting for curl command:".red(), e);
            let _ = bash_cmd.kill();
            return 1;
        }
    };

    if !curl_status.success() {
        eprintln!(
            "{} {}",
            "curl command failed with status:".red(),
            curl_status.code().unwrap_or(-1)
        );
        let _ = bash_cmd.kill();
        return 1;
    }

    let output = match bash_cmd.wait_with_output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{} {}", "error waiting for bash command:".red(), e);
            return 1;
        }
    };

    if output.status.success() {
        println!("oh-my-zsh {}", "installed successfully".green());
        return 0;
    } else {
        eprintln!(
            "{} command `{}` failed:\nstdout: {}\nstderr: {}",
            "error downloading oh-my-zsh:".red(),
            "curl ... | bash".red(),
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return 1;
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
