pub struct UserCfg {
    name: String,
    home_dir: String,
}

impl UserCfg {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            home_dir: "".to_string(),
        }
    }

    pub fn set_name(&mut self, name: String) -> i8 {
        if name != "" {
            self.name = name;
            return 0;
        } else {
            return 1;
        }
    }

    pub fn set_home(&mut self, home: String) -> i8 {
        if home != "" {
            self.home_dir = home;
            return 0;
        } else {
            return 1;
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_home(&self) -> String {
        self.home_dir.clone()
    }
}