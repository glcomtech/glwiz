/*
gnulinwiz AKA GNU/Linux Config Wizard: The ultimate post-installation setup assistant for Linux,
streamlining your configuration process with ease and precision.

Copyright (C) 2025  Andrew Kushyk

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

/// struct for storing username and home path
pub struct UserCfg {
    name: String,
    home_dir: String,
} // UserCfg

impl UserCfg {
    /// creates new user
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            home_dir: "".to_string(),
        }
    }

    /// sets name for new user
    pub fn set_name(&mut self, name: String) -> i8 {
        if name != "" {
            self.name = name;
            return 0;
        } else {
            return 1;
        }
    }

    /// sets home path for user
    pub fn set_home(&mut self, home: String) -> i8 {
        if home != "" {
            self.home_dir = home;
            return 0;
        } else {
            return 1;
        }
    }

    /// gets username
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// gets home path
    pub fn get_home(&self) -> String {
        self.home_dir.clone()
    }
} // impl UserCfg
