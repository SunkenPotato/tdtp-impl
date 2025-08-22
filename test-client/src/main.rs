use std::{
    process::exit,
    thread::{sleep, spawn},
    time::Duration,
};

use esp32_server::{client::status, consts::StatusData, server::Server};

fn main() {
    let serv = spawn(|| Server::run([127, 0, 0, 1], 8000, 4).unwrap());
    sleep(Duration::from_secs(1));
    let rsp = status(
        StatusData::default(),
        "127.0.0.1:8000",
        Some("Hello".as_bytes()),
    )
    .unwrap();

    exit(0);
}
