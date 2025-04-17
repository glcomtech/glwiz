pub mod functionality;
use functionality::prog_fun::{
    check_sw_install_type, default_sw_package, print_license_info, print_setup_status_success,
    set_sw_list, validate_root_priviliges,
};
use functionality::setup_fun::{
    install_omz, install_zsh_autosuggestions, install_zsh_syntax_highlighting, iptables_file_setup,
    iptables_rules_setup, software_setup, user_config_setup, validate_env_var,
    validate_task_status,
};
use functionality::user_cfg::UserCfg;

/// default function for setting up necessary tools
pub fn gnu_linux_setup() {
    // prints license info
    print_license_info();

    // validates root priviliges
    validate_root_priviliges();

    // sets up gnu/linux username and home path
    let mut user_cfg = UserCfg::new();
    let user_name = validate_env_var("USER");
    validate_task_status(user_cfg.set_name(user_name));
    let home_dir = validate_env_var("HOME");
    validate_task_status(user_cfg.set_home(home_dir));

    // sets up iptables firewall and initializes rules
    validate_task_status(iptables_file_setup());
    validate_task_status(iptables_rules_setup());

    // sets up a list of sw to download
    validate_task_status(if check_sw_install_type() {
        software_setup(&set_sw_list())
    } else {
        software_setup(&default_sw_package())
    });

    // sets up zsh shell
    validate_task_status(install_omz(user_cfg.get_name()));
    validate_task_status(install_zsh_autosuggestions(
        user_cfg.get_name(),
        user_cfg.get_home(),
    ));
    validate_task_status(install_zsh_syntax_highlighting(
        user_cfg.get_name(),
        user_cfg.get_home(),
    ));
    validate_task_status(user_config_setup(
        "../../configs/.zshrc".to_string(),
        user_cfg.get_home(),
    ));

    // sets up vim configuration
    validate_task_status(user_config_setup(
        "../../configs/.vimrc".to_string(),
        user_cfg.get_home(),
    ));

    print_setup_status_success();
} // gnu_linux_setup()
