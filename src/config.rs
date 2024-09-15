use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_name: String,
    pub password: String,
}

impl Config {
    pub fn load() -> Config {
        confy::load("cprs", "config")
            .with_context(|| "Cannot get config file for cprs")
            .unwrap()
    }
    pub fn save(&self) {
        confy::store("cprs", "config", self.clone())
            .with_context(|| "Cannot save config file for cprs")
            .unwrap()
    }
}
