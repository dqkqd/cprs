use crate::{build::bundler, task::Task};

use super::{cmd::Build, Run};
use anyhow::{Context, Result};
use tokio::{fs, process};

impl Run for Build {
    async fn run(&self) -> Result<()> {
        let task = Task::from_current_dir().await?;
        if task.submit_file.is_file() {
            fs::remove_file(&task.submit_file).await?;
        }
        let code = bundler::bundle_task(&task)?;
        fs::write(&task.submit_file, code).await?;

        // rerun test to make sure everything ok
        let submit_bin = task
            .submit_file
            .file_stem()
            .and_then(|f| f.to_str())
            .unwrap();
        let out = process::Command::new("cargo")
            .args(["test", "--color", "always", "--bin", submit_bin])
            .output()
            .await
            .with_context(|| format!("Failed to execute test for `{}`", submit_bin))?;
        println!("{}", String::from_utf8(out.stderr)?);
        println!("{}", String::from_utf8(out.stdout)?);
        println!(
            "You can rerun this using:\n  cargo test --bin {}",
            submit_bin
        );
        Ok(())
    }
}
