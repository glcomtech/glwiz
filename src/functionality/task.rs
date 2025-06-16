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

use crate::functionality::prog_fun::print_setup_status_failed;
use std::process::exit;

/// Validates the status of a task and exits the program on failure.
///
/// Checks the provided status code and, if it is non-zero, prints a failure message
/// using `print_setup_status_failed` and terminates the program with the status code
/// as the exit code. This function ensures that any task failure halts the configuration
/// process to prevent further errors.
///
/// # Arguments
/// * `status` - The status code of a task (0 for success, non-zero for failure).
///
/// # Panics
/// Exits the program with the provided `status` code (cast to `i32`) if it is non-zero.
///
/// # Examples
/// ```
/// // Successful task
/// validate_task_status(0); // Continues execution
///
/// // Failed task
/// // validate_task_status(1); // Prints failure message and exits with code 1
/// ```
pub fn validate_task_status(status: i8) {
    if status != 0 {
        print_setup_status_failed();
        exit(status as i32);
    }
}
