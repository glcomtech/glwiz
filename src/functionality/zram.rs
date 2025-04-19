use colored::Colorize;
use std::process::Command;

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
