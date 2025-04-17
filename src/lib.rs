pub mod functionality;
use functionality::prog_fun::{
    check_sw_install_type, default_package, print_license_info, print_setup_status_success,
    set_sw_list, validate_root_priviliges,
};
use functionality::setup_fun::{iptables_setup, software_setup, validate_task_status};

pub fn gnu_linux_setup() {
    print_license_info();
    validate_root_priviliges();

    validate_task_status(if check_sw_install_type() {
        software_setup(&set_sw_list())
    } else {
        software_setup(&default_package())
    });

    validate_task_status(iptables_setup());

    print_setup_status_success();
}
