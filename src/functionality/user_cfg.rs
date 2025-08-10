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

/// Stores user configuration data for the GNU/Linux Config Wizard.
///
/// This struct holds user-specific information, such as the username and home directory, used
/// in the "gnulinwiz" project to configure user environments during post-installation setup.
/// It provides methods to set and retrieve these values, ensuring valid data through checks
/// for non-empty strings and existing directories. The struct is critical for tasks like
/// setting up shell configurations, copying user files, or installing software tailored to
/// the user’s environment.
///
/// # Fields
/// * `name` - The username as a `String` (e.g., `"user"`).
/// * `home_dir` - The user’s home directory path as a `String` (e.g., `"/home/user"`).
///
/// # Example
/// ```
/// use gnulinwiz::functionality::user_cfg::UserCfg;
/// let mut user_cfg = UserCfg::new();
/// user_cfg.set_name("user").unwrap();
/// user_cfg.set_home("/home/user").unwrap();
/// assert_eq!(user_cfg.get_name(), "user");
/// assert_eq!(user_cfg.get_home(), "/home/user");
/// ```
///
/// # See Also
/// - `env::get_env_var`: Used to retrieve environment variables like `USER` or `HOME`.
/// - `configs::user_config_setup`: Uses this struct for user-specific configuration files.
pub struct UserCfg {
    name: String,
    home_dir: String,
}

impl UserCfg {
    /// Creates a new, empty `UserCfg` instance.
    ///
    /// This method initializes a `UserCfg` struct with empty `name` and `home_dir` fields.
    /// It is typically followed by calls to `set_name` and `set_home` to populate the struct
    /// with valid user data.
    ///
    /// # Returns
    /// A `UserCfg` instance with empty fields.
    ///
    /// # Example
    /// ```
    /// use gnulinwiz::functionality::user_cfg::UserCfg;
    /// let user_cfg = UserCfg::new();
    /// assert_eq!(user_cfg.get_name(), "");
    /// assert_eq!(user_cfg.get_home(), "");
    /// ```
    pub fn new() -> Self {
        Self {
            name: String::new(),
            home_dir: String::new(),
        }
    }

    /// Sets the username for the `UserCfg` instance.
    ///
    /// This method updates the `name` field with the provided username, ensuring it is non-empty.
    /// It is used in the "gnulinwiz" project to store the username for tasks like changing the
    /// default shell or configuring user-specific files.
    ///
    /// # Arguments
    /// * `name` - The username to set (e.g., `"user"`).
    ///
    /// # Returns
    /// * `Ok(())` - The username was successfully set.
    /// * `Err(String)` - An error message if the username is empty.
    ///
    /// # Errors
    /// Returns an error if the provided `name` is an empty string.
    ///
    /// # Example
    /// ```
    /// use gnulinwiz::functionality::user_cfg::UserCfg;
    /// let mut user_cfg = UserCfg::new();
    /// let result = user_cfg.set_name("user");
    /// assert!(result.is_ok());
    /// assert_eq!(user_cfg.get_name(), "user");
    /// ```
    pub fn set_name(&mut self, name: &str) -> Result<(), String> {
        if name.is_empty() {
            Err("Username cannot be empty".to_string())
        } else {
            self.name = name.to_string();
            Ok(())
        }
    }

    /// Sets the home directory for the `UserCfg` instance.
    ///
    /// This method updates the `home_dir` field with the provided path, ensuring it is non-empty
    /// and corresponds to an existing directory. It is used in the "gnulinwiz" project for tasks
    /// like copying configuration files to the user’s home directory.
    ///
    /// # Arguments
    /// * `home` - The home directory path to set (e.g., `"/home/user"`).
    ///
    /// # Returns
    /// * `Ok(())` - The home directory was successfully set.
    /// * `Err(String)` - An error message if the path is empty or does not exist.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The provided `home` path is an empty string.
    /// - The path does not correspond to an existing directory.
    ///
    /// # Example
    /// ```
    /// use gnulinwiz::functionality::user_cfg::UserCfg;
    /// let mut user_cfg = UserCfg::new();
    /// let result = user_cfg.set_home("/home/user");
    /// if result.is_ok() {
    ///     assert_eq!(user_cfg.get_home(), "/home/user");
    /// }
    /// ```
    pub fn set_home(&mut self, home: &str) -> Result<(), String> {
        if home.is_empty() {
            Err("Home directory cannot be empty".to_string())
        } else if !std::path::Path::new(home).exists() {
            Err(format!("Home directory {} does not exist", home))
        } else {
            self.home_dir = home.to_string();
            Ok(())
        }
    }

    /// Retrieves the stored username.
    ///
    /// This method returns a reference to the `name` field, used in the "gnulinwiz" project to
    /// access the username for configuration tasks like setting the default shell.
    ///
    /// # Returns
    /// A string slice (`&str`) containing the username.
    ///
    /// # Example
    /// ```
    /// use gnulinwiz::functionality::user_cfg::UserCfg;
    /// let mut user_cfg = UserCfg::new();
    /// user_cfg.set_name("user").unwrap();
    /// assert_eq!(user_cfg.get_name(), "user");
    /// ```
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Retrieves the stored home directory.
    ///
    /// This method returns a reference to the `home_dir` field, used in the "gnulinwiz" project
    /// to access the home directory for tasks like copying configuration files.
    ///
    /// # Returns
    /// A string slice (`&str`) containing the home directory path.
    ///
    /// # Example
    /// ```
    /// use gnulinwiz::functionality::user_cfg::UserCfg;
    /// let mut user_cfg = UserCfg::new();
    /// user_cfg.set_home("/home/user").unwrap();
    /// assert_eq!(user_cfg.get_home(), "/home/user");
    /// ```
    pub fn get_home(&self) -> &str {
        &self.home_dir
    }
}
