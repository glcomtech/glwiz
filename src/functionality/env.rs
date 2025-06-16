/*
 *  gnulinwiz AKA GNU/Linux Config Wizard: The ultimate post-installation setup assistant for Linux,
 *  streamlining your configuration process with ease and precision.
 *
 *  Copyright (C) 2025  Andrew Kushyk
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use colored::Colorize;
use std::env::var;

/// Retrieves the value of a specified environment variable.
///
/// This function attempts to read the value of the given environment variable and returns it as an
/// owned `String` if found. If the variable is not set or an error occurs, it prints an error message
/// to stderr and returns `None`.
///
/// # Arguments
/// * `env_var` - The name of the environment variable to retrieve (e.g., "USER", "HOME").
///
/// # Returns
/// * `Some(String)` containing the variable's value if it exists.
/// * `None` if the variable is not set or an error occurs while reading it.
///
/// # Examples
/// ```
/// let user = get_env_var("USER");
/// assert!(user.is_some());
/// ```
pub fn get_env_var(env_var: &str) -> Option<String> {
    match var(env_var) {
        Ok(value) => Some(value),
        Err(e) => {
            eprintln!(
                "{} failed to get environment variable '{}': {}",
                "error:".red(),
                env_var,
                e
            );
            None
        }
    }
}
