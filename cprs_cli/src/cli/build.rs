use crate::{build::bundler, task::Task};

use super::{cmd::Build, Run};
use anyhow::Result;

impl Run for Build {
    async fn run(&self) -> Result<()> {
        let task = Task::from_current_dir().await?;
        let code = bundler::bundle_task(&task)?;
        Ok(())
    }
}
