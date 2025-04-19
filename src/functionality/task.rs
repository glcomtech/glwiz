use crate::functionality::prog_fun::print_setup_status_failed;
use std::process::exit;

/// validates task status
pub fn validate_task_status(status: i8) {
    if status != 0 {
        print_setup_status_failed();
        exit(status as i32);
    }
} // validate_task_status()
