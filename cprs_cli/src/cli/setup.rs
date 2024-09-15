use crate::config::Config;

use super::{cmd::Setup, Run};
pub use anyhow::Result;

impl Run for Setup {
    fn run(&self) -> Result<()> {
        Config::default().save();
        Ok(())
    }
}
