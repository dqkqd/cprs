mod cd;
mod cmd;
mod init;
mod list;
mod listen;
mod setup;

pub use cmd::Cmd;

pub use anyhow::Result;

trait Run {
    fn run(&self) -> Result<()>;
}
