use crate::{task::Task, DaemonizeClipboard};

use super::{cmd::Submit, Run};
use anyhow::Result;

impl Run for Submit {
    async fn run(&self) -> Result<()> {
        // currently just copy content from submit_file
        let task = Task::from_current_dir().await?;
        DaemonizeClipboard::try_spawn_copy_process(&task.submit_file)?;
        Ok(())
    }
}
