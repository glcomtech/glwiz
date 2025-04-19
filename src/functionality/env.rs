/*
gnulinwiz AKA GNU/Linux Config Wizard: The ultimate post-installation setup assistant for Linux,
streamlining your configuration process with ease and precision.

Copyright (C) 2025  Andrew Kushyk

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use colored::Colorize;
use std::env::var;

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
} // validate_env_var
