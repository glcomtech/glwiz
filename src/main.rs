mod prog_fun;
mod setup_fun;

use prog_fun::{check_sw_install_type, default_package, print_license_info, print_setup_status_success, set_sw_list, validate_root_priviliges};
use setup_fun::{software_setup, validate_task_status};

fn main() {
    print_license_info();
    validate_root_priviliges();

    validate_task_status(
        if check_sw_install_type() {
            software_setup(&set_sw_list())
        } else {
            software_setup(&default_package())
        }
    );

    print_setup_status_success();
}
