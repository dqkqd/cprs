use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_name: String,
    pub password: String,
    pub workspace: PathBuf,
    pub history: PathBuf,
    pub history_limit: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_name: "".into(),
            password: "".into(),
            workspace: std::env::current_dir()
                .with_context(|| "Cannot get current directory")
                .unwrap()
                .join("contests"),
            history: dirs::data_dir()
                .with_context(|| "Cannot get data directory for saving history file")
                .unwrap()
                .join("history.json"),
            history_limit: 1000,
        }
    }
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
    pub fn ask_user_name() {
        todo!()
    }
    pub fn ask_password() {
        todo!()
    }
}
