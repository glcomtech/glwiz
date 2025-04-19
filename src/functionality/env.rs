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
