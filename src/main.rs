mod prog_fun;
mod setup_fun;

use prog_fun::{print_license_info, print_setup_status_failed, print_setup_status_success};

fn main() {
    print_license_info();
    print_setup_status_success();
}
