use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_name: String,
    pub password: String,
}

impl Config {
    pub fn load() -> Result<Config, confy::ConfyError> {
        confy::load("cprs", "config")
    }
    pub fn save(self) -> Result<(), confy::ConfyError> {
        confy::store("cprs", "config", self)
    }
}
