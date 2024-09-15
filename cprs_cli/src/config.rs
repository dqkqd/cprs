use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_name: String,
    pub password: String,
    pub workspace: PathBuf,
    pub templates: PathBuf,
    pub algo_lib: PathBuf,

    // un important config
    pub history: PathBuf,
    pub history_limit: usize,
    pub competitive_companion_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        let current_dir = std::env::current_dir()
            .with_context(|| "Cannot get current directory")
            .unwrap();
        let data_dir = dirs::data_dir()
            .with_context(|| "Cannot get data directory for saving history file")
            .unwrap();
        Self {
            user_name: "".into(),
            password: "".into(),
            workspace: current_dir.join("contests"),
            templates: current_dir.join("templates"),
            algo_lib: current_dir.join("algo"),
            history: data_dir.join("history.json"),
            history_limit: 1000,
            competitive_companion_port: 27121,
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
