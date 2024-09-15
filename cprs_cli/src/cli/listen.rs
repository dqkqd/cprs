use crate::listener;

use super::{cmd::Listen, Run};
use anyhow::Result;

impl Run for Listen {
    async fn run(&self) -> Result<()> {
        listener::listen().await
    }
}
