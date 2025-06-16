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

/// A structure for storing a user's username and home directory path.
///
/// This struct holds references to the username and home directory path with a lifetime `'a`,
/// used to manage user-specific configuration data in the GNU/Linux Config Wizard.
pub struct UserCfg<'a> {
    name: &'a str,
    home_dir: &'a str,
}

impl<'a> UserCfg<'a> {
    /// Creates a new `UserCfg` instance with empty username and home directory.
    ///
    /// Initializes the struct with empty string references, ready to be populated
    /// using `set_name` and `set_home`.
    ///
    /// # Returns
    /// A new `UserCfg` instance with empty fields.
    ///
    /// # Examples
    /// ```
    /// let user = UserCfg::new();
    /// assert_eq!(user.get_name(), "");
    /// assert_eq!(user.get_home(), "");
    /// ```
    pub fn new() -> Self {
        Self {
            name: "",
            home_dir: "",
        }
    }

    /// Sets the username for the `UserCfg` instance.
    ///
    /// Updates the `name` field with the provided username if it is non-empty.
    ///
    /// # Arguments
    /// * `name` - The username to set, as a string slice with lifetime `'a`.
    ///
    /// # Returns
    /// * `0` if the username is set successfully (non-empty).
    /// * `1` if the provided username is empty.
    ///
    /// # Examples
    /// ```
    /// let mut user = UserCfg::new();
    /// let result = user.set_name("alice");
    /// assert_eq!(result, 0);
    /// assert_eq!(user.get_name(), "alice");
    /// ```
    pub fn set_name(&mut self, name: &'a str) -> i8 {
        if name != "" {
            self.name = name;
            return 0;
        } else {
            return 1;
        }
    }

    /// Sets the home directory path for the `UserCfg` instance.
    ///
    /// Updates the `home_dir` field with the provided path if it is non-empty.
    ///
    /// # Arguments
    /// * `home` - The home directory path to set, as a string slice with lifetime `'a`.
    ///
    /// # Returns
    /// * `0` if the home directory path is set successfully (non-empty).
    /// * `1` if the provided path is empty.
    ///
    /// # Examples
    /// ```
    /// let mut user = UserCfg::new();
    /// let result = user.set_home("/home/alice");
    /// assert_eq!(result, 0);
    /// assert_eq!(user.get_home(), "/home/alice");
    /// ```
    pub fn set_home(&mut self, home: &'a str) -> i8 {
        if home != "" {
            self.home_dir = home;
            return 0;
        } else {
            return 1;
        }
    }

    /// Retrieves the stored username.
    ///
    /// # Returns
    /// A string slice containing the username.
    ///
    /// # Examples
    /// ```
    /// let mut user = UserCfg::new();
    /// user.set_name("alice");
    /// assert_eq!(user.get_name(), "alice");
    /// ```
    pub fn get_name(&self) -> &str {
        self.name
    }

    /// Retrieves the stored home directory path.
    ///
    /// # Returns
    /// A string slice containing the home directory path.
    ///
    /// # Examples
    /// ```
    /// let mut user = UserCfg::new();
    /// user.set_home("/home/alice");
    /// assert_eq!(user.get_home(), "/home/alice");
    /// ```
    pub fn get_home(&self) -> &str {
        self.home_dir
    }
}
