use anyhow::{bail, Result};
use tokio::process;

use crate::task::Task;

pub struct Submitter<'a> {
    pub task: &'a Task,
    pub commit: bool,
}

impl<'a> Submitter<'a> {
    pub async fn submit(&self) -> Result<()> {
        if self.commit {
            self.git_commit().await?;
        }
        Ok(())
    }

    async fn git_commit(&self) -> Result<()> {
        // check status before submitting to make sure not dirty files are committed
        let out = process::Command::new("git")
            .args(["diff", "--staged"])
            .output()
            .await?;
        if !out.stderr.is_empty() || !out.stdout.trim_ascii().is_empty() {
            bail!("Root folder is dirty, please clean before submitting");
        }

        // change branch before submitting
        process::Command::new("git")
            .args(["checkout", "contest"])
            .output()
            .await?;

        let mut child = process::Command::new("git")
            .args(["add", self.task.task_folder.to_str().unwrap()])
            .spawn()?;
        child.wait().await?;

        let utc: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let msg = format!("[submit]: {} at {}", self.task.task_name, utc);
        process::Command::new("git")
            .args(["commit", "-m", &msg])
            .spawn()?;

        Ok(())
    }
}
