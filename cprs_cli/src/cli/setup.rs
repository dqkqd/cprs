use crate::config::Config;

use super::{cmd::Setup, Run};
use anyhow::Result;

impl Run for Setup {
    async fn run(&self) -> Result<()> {
        Config::default().save();
        Ok(())
    }
}
