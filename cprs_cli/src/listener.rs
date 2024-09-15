use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use anyhow::Context;

use crate::{config::Config, task::{Task, TaskRaw}};

pub fn listen() {
    let config = Config::load();
    let address = format!("127.0.0.1:{}", config.competitive_companion_port);
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let task_raw_description = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .skip_while(|line| !line.is_empty())
        .last();

    let task_raw: TaskRaw = match task_raw_description {
        Some(description) => serde_json::from_str(&description).unwrap(),
        None => todo!(),
    };
    let task = Task::from(task_raw);
    task.setup()
        .with_context(|| format!("Cannot setup task {}", &task.raw.name))
        .unwrap();
}
