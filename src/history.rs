use std::collections::VecDeque;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{config::Config, task::Task};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    tasks: VecDeque<Task>,
}

impl History {
    fn load() -> anyhow::Result<History> {
        let config = Config::load();
        match std::fs::read_to_string(&config.history) {
            Ok(content) => serde_json::from_str(&content).with_context(|| {
                format!(
                    "Cannot parse history file, please check `{}`",
                    &config.history.display()
                )
            }),
            Err(_) => Ok(History::default()),
        }
    }
    fn save(&self) -> anyhow::Result<()> {
        let config = Config::load();
        std::fs::write(&config.history, serde_json::to_string(self)?)
            .with_context(|| "Cannot save to history file")?;
        Ok(())
    }
    pub fn add_task(task: Task) -> anyhow::Result<()> {
        let config = Config::load();
        let mut history = History::load()?;
        match history.tasks.iter().position(|t| t == &task) {
            Some(0) => {
                // already added at the latest position, do nothing
                return Ok(());
            }
            Some(index) => {
                // duplicated, remove old entry
                history.tasks.remove(index);
            }
            _ => {
                // not existed, do nothing
            }
        }
        if history.tasks.len() >= config.history_limit {
            history.tasks.pop_back();
        }
        history.tasks.push_front(task);
        history.save()?;
        Ok(())
    }
    fn get_latest_tasks(len: usize) -> Vec<Task> {
        History::load()
            .unwrap_or_default()
            .tasks
            .into_iter()
            .take(len)
            .collect()
    }
}
