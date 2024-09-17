mod build;
mod cd;
mod cmd;
mod init;
mod list;
mod listen;
mod setup;

pub use cmd::Cmd;

use anyhow::Result;

trait Run {
    async fn run(&self) -> Result<()>;
}
