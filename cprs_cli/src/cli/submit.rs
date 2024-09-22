use crate::{submitter::Submitter, task::Task, DaemonizeClipboard};

use super::{
    cmd::{Build, Submit},
    Run,
};
use anyhow::Result;

impl Run for Submit {
    async fn run(&self) -> Result<()> {
        // rebuild before submitting
        Build {}.run().await?;
        let task = Task::from_current_dir().await?;
        DaemonizeClipboard::try_spawn_copy_process(&task.submit_file)?;

        let submitter = Submitter { task: &task };
        submitter.git_submit().await?;
        Ok(())
    }
}
