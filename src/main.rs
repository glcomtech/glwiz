mod prog_fun;
mod setup_fun;

use prog_fun::{print_license_info, print_setup_status_success, validate_root_priviliges};
use setup_fun::{software_setup, validate_task_status};

fn main() {
    print_license_info();
    validate_root_priviliges();

    validate_task_status(software_setup());

    print_setup_status_success();
}
