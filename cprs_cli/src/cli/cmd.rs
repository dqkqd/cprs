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
    pub fn entry_point() -> Result<()> {
        Cmd::parse().run()
    }
}

impl Run for Cmd {
    fn run(&self) -> Result<()> {
        match self {
            Cmd::Init(init) => init.run()?,
            Cmd::Listen(listen) => listen.run()?,
            Cmd::List(list) => list.run()?,
            Cmd::Cd(cd) => cd.run()?,
            Cmd::Setup(setup) => setup.run()?,
        }
        Ok(())
    }
}
