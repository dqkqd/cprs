use crate::listener;

use super::{cmd::Listen, Run};
use anyhow::Result;

impl Run for Listen {
    fn run(&self) -> Result<()> {
        listener::listen();
        Ok(())
    }
}
