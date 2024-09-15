use clap::Parser;

use super::Run;
use anyhow::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Cmd {
    Setup(Setup),
    Init(Init),
    Listen(Listen),
    List(List),
    Cd(Cd),
}

#[derive(Parser)]
pub struct Setup;

#[derive(Parser)]
pub struct Init;

#[derive(Parser)]
pub struct Listen;

#[derive(Parser)]
pub struct List {
    pub task_count: usize,
}

#[derive(Parser)]
pub struct Cd {
    pub task_id: usize,
}

impl Cmd {
    pub async fn entry_point() -> Result<()> {
        Cmd::parse().run().await
    }
}

impl Run for Cmd {
    async fn run(&self) -> Result<()> {
        match self {
            Cmd::Init(init) => init.run().await?,
            Cmd::Listen(listen) => listen.run().await?,
            Cmd::List(list) => list.run().await?,
            Cmd::Cd(cd) => cd.run().await?,
            Cmd::Setup(setup) => setup.run().await?,
        }
        Ok(())
    }
}
