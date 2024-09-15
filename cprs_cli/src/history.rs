use std::collections::VecDeque;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    config::Config,
    task::{Task, TaskRaw},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    tasks: VecDeque<TaskRaw>,
}

impl History {
    async fn load() -> Result<History> {
        let config = Config::load();
        match fs::read_to_string(&config.history).await {
            Ok(content) => serde_json::from_str(&content).with_context(|| {
                format!(
                    "Cannot parse history file, please check `{}`",
                    &config.history.display()
                )
            }),
            Err(_) => Ok(History::default()),
        }
    }
    async fn save(&self) -> Result<()> {
        let config = Config::load();
        fs::write(&config.history, serde_json::to_string(self)?)
            .await
            .with_context(|| "Cannot save to history file")?;
        Ok(())
    }
    pub async fn add_task(task: Task) -> Result<()> {
        let mut history = History::load().await?;
        match history.tasks.iter().position(|t| t.url == task.raw.url) {
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
        if history.tasks.len() >= task.config.history_limit {
            history.tasks.pop_back();
        }
        history.tasks.push_front(task.raw);
        history.save().await?;
        Ok(())
    }
    pub async fn get_latest_tasks(count: usize) -> Vec<Task> {
        History::load()
            .await
            .unwrap_or_default()
            .tasks
            .into_iter()
            .take(count)
            .map(Task::from)
            .collect()
    }
    pub async fn get_task(task_id: usize) -> Result<Task> {
        History::load()
            .await
            .unwrap_or_default()
            .tasks
            .get(task_id)
            .cloned()
            .map(Task::from)
            .with_context(|| format!("There is no task with id: {task_id}"))
    }
}
