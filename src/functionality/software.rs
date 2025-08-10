/*
 * GLWiz - The ultimate post-installation setup assistant for GNU/Linux popular disros,
 * streamlining your configuration process with ease and precision.
 * 
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use colored::Colorize;
use std::process::Command;

/// Installs software packages using the distribution’s package manager.
///
/// This function installs a list of software packages on a GNU/Linux system by selecting the
/// appropriate package manager based on the detected distribution (Arch, Debian, or Fedora).
/// It uses `sudo` to execute commands like `pacman`, `apt`, or `dnf` with distribution-specific
/// arguments for non-interactive installation. The function is a core component of the
/// "glwiz" project’s post-installation setup, enabling automated software installation
/// for user-specified or default package lists. It logs the command being run and reports
/// success or failure with detailed error messages.
///
/// # Arguments
/// * `packages` - A slice of package names to install (e.g., `&["firefox", "vim"]`).
/// * `distro` - The Linux distribution identifier (e.g., `"arch"`, `"debian"`, `"fedora"`).
///
/// # Returns
/// * `0` - All packages were successfully installed.
/// * `1` - An error occurred, such as an unsupported distribution, failed command, or package installation error.
///
/// # Errors
/// Returns `1` if:
/// - The `distro` is not supported (i.e., not `"arch"`, `"debian"`, or `"fedora"`).
/// - The package manager command fails to execute (e.g., `sudo` or the package manager is not installed).
/// - The installation command exits with a non-zero status, indicating issues like unavailable packages or network errors.
///
/// # Example
/// ```
/// use glwiz::functionality::software::software_setup;
/// let packages = &["firefox", "vim"];
/// let result = software_setup(packages, "debian");
/// assert_eq!(result, 0); // Packages installed successfully
/// ```
///
/// # See Also
/// - `prog_fun::default_sw_package`: Provides the default package list.
/// - `prog_fun::set_sw_list`: Collects custom package lists from user input.
/// - `commands::run_sudo_command`: Used to execute package manager commands with `sudo`.
pub fn software_setup(packages: &[&str], distro: &str) -> i8 {
    let (cmd, args) = match distro {
        "arch" => ("pacman", vec!["-Sy", "--noconfirm"]),
        "debian" => ("apt", vec!["install", "-y"]),
        "fedora" => ("dnf", vec!["install", "-y"]),
        _ => {
            eprintln!("{} Unsupported distribution: {}", "error:".red(), distro);
            return 1;
        }
    };

    let mut command = Command::new("sudo");
    command.arg(cmd).args(&args).args(packages);
    println!(
        "Running: {} {} {}",
        cmd,
        args.join(" "),
        packages.join(" ").green()
    );

    match command.output() {
        Ok(output) if output.status.success() => {
            println!("Software {}.", "installed".green());
            0
        }
        Ok(output) => {
            eprintln!(
                "{} Software installation failed:\nstdout: {}\nstderr: {}",
                "error:".red(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            1
        }
        Err(e) => {
            eprintln!("{} Failed to run {}: {}", "error:".red(), cmd, e);
            1
        }
    }
}
