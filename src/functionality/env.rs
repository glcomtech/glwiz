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
use std::env;

/// Retrieves the value of a specified environment variable for system configuration.
///
/// This function fetches the value of an environment variable (e.g., `USER`, `HOME`) and is used
/// in the "gnulinwiz" project to gather user-specific data during post-installation setup. It ensures
/// robust error handling by returning a descriptive error message if the variable is unset or
/// invalid. The function is critical for tasks like configuring user environments and detecting
/// system settings, providing a safe interface to `std::env::var`.
///
/// # Arguments
/// * `env_var` - The name of the environment variable to retrieve (e.g., `"USER"`, `"HOME"`).
///
/// # Returns
/// * `Ok(String)` - The value of the environment variable as a `String`.
/// * `Err(String)` - An error message if the variable is unset or contains invalid Unicode.
///
/// # Errors
/// Returns an error if:
/// - The environment variable is not set.
/// - The variable’s value contains invalid Unicode, preventing conversion to a `String`.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::env::get_env_var;
/// match get_env_var("USER") {
///     Ok(username) => println!("Current user: {}", username),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # See Also
/// - `user_cfg::UserCfg`: Uses this function to set user-specific configuration data.
/// - `std::env::var`: The underlying Rust function for environment variable access.
pub fn get_env_var(env_var: &str) -> Result<String, String> {
    env::var(env_var).map_err(|e| format!("{} failed to get '{}': {}", "error:".red(), env_var, e))
}
