use crate::{history::History, utils::println_to_console};

use super::{cmd::List, Run};
pub use anyhow::Result;

impl Run for List {
    fn run(&self) -> Result<()> {
        let tasks = History::get_latest_tasks(self.task_count);
        println_to_console(format!("Showing {} latest tasks:", self.task_count));
        tasks
            .iter()
            .enumerate()
            .map(|(id, task)| format!(" Id {:>2}: {}", id, task.summary()))
            .for_each(println_to_console);
        Ok(())
    }
}
