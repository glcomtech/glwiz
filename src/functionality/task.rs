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

/// Represents the outcome of a single setup task in the GNU/Linux Config Wizard.
///
/// This struct captures the result of a configuration task, such as installing software or setting
/// up iptables, in the "gnulinwiz" project. It stores a status code indicating success or failure
/// and a descriptive message for logging or error reporting. The struct is used to collect and
/// validate task outcomes during the post-installation setup process, ensuring comprehensive
/// tracking of the configuration workflow.
///
/// # Fields
/// * `status` - An `i8` status code: `0` for success, non-zero for failure (typically `1`).
/// * `message` - A `String` describing the task (e.g., "Software installation").
///
/// # Example
/// ```
/// use gnulinwiz::functionality::task::TaskResult;
/// let task = TaskResult {
///     status: 0,
///     message: String::from("Software installation"),
/// };
/// assert_eq!(task.status, 0);
/// assert_eq!(task.message, "Software installation");
/// ```
///
/// # See Also
/// - `validate_task_statuses`: Uses this struct to validate a collection of task results.
/// - `lib.rs`: Orchestrates tasks and collects `TaskResult` instances.
#[derive(Debug)]
pub struct TaskResult {
    pub status: i8,
    pub message: String,
}

/// Validates a collection of task results to determine setup success.
///
/// This function evaluates a vector of `TaskResult` instances to check if all setup tasks in the
/// "gnulinwiz" project completed successfully. If any task failed (non-zero status), it logs the
/// errors with descriptive messages and returns `false`. Otherwise, it returns `true`, indicating
/// a successful setup. The function is used to provide comprehensive error reporting and ensure
/// the integrity of the post-installation configuration process.
///
/// # Arguments
/// * `tasks` - A `Vec<TaskResult>` containing the results of setup tasks.
///
/// # Returns
/// * `true` - All tasks completed successfully (all statuses are `0`).
/// * `false` - One or more tasks failed (non-zero statuses), with errors logged to stderr.
///
/// # Example
/// ```
/// use gnulinwiz::functionality::task::{TaskResult, validate_task_statuses};
/// let tasks = vec![
///     TaskResult { status: 0, message: String::from("Task 1") },
///     TaskResult { status: 1, message: String::from("Task 2") },
/// ];
/// let result = validate_task_statuses(tasks);
/// assert_eq!(result, false); // Failed due to Task 2
/// ```
///
/// # See Also
/// - `TaskResult`: The struct representing individual task outcomes.
/// - `prog_fun::print_setup_status_success`: Called on successful validation.
/// - `prog_fun::print_setup_status_failed`: Called on failed validation.
pub fn validate_task_statuses(tasks: Vec<TaskResult>) -> bool {
    let errors: Vec<_> = tasks.into_iter().filter(|t| t.status != 0).collect();

    if errors.is_empty() {
        true
    } else {
        eprintln!("{} Setup failed with errors:", "error:".red());
        for task in errors {
            eprintln!("- {}: status {}", task.message, task.status);
        }
        false
    }
}
