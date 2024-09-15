use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use anyhow::Context;

use crate::task::Task;

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:27121").unwrap();
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

    let task: Task = match task_raw_description {
        Some(description) => serde_json::from_str(&description).unwrap(),
        None => todo!(),
    };
    task.setup()
        .with_context(|| format!("Cannot setup task {}", &task.name))
        .unwrap();
}
