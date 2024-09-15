use anyhow::{Context, Result};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::{
    config::Config,
    task::{Task, TaskRaw},
};

pub async fn listen() -> Result<()> {
    let config = Config::load();
    let address = format!("127.0.0.1:{}", config.competitive_companion_port);
    let listener = TcpListener::bind(address).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await.unwrap();
        });
    }
}

async fn process(mut stream: TcpStream) -> Result<()> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).await?;
    let task_raw_description = buf
        .lines()
        .skip_while(|line| !line.is_empty())
        .last()
        .with_context(|| "Cannot get task description")?;
    let task_raw: TaskRaw = serde_json::from_str(task_raw_description)?;
    let task = Task::from(task_raw);
    task.setup()
        .await
        .with_context(|| format!("Cannot setup task {}", &task.raw.name))
}
