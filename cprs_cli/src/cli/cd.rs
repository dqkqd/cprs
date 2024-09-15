use crate::{history::History, utils::println_to_console};

use super::{cmd::Cd, Run};
use anyhow::Result;

impl Run for Cd {
    async fn run(&self) -> Result<()> {
        let task = History::get_task(self.task_id).await?;
        println_to_console(task.task_folder.display());
        Ok(())
    }
}
