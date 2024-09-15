use clap::Parser;

use super::Run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Cmd {
    Init(Init),
    Listen(Listen),
    List(List),
    Cd(Cd),
}

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
    pub fn entry_point() -> anyhow::Result<()> {
        Cmd::parse().run()
    }
}

impl Run for Cmd {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Cmd::Init(init) => init.run()?,
            Cmd::Listen(listen) => listen.run()?,
            Cmd::List(list) => list.run()?,
            Cmd::Cd(cd) => cd.run()?,
        }
        Ok(())
    }
}
