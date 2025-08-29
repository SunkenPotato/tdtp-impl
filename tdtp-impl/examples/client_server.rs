use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr},
    sync::{
        Arc,
        mpsc::{Receiver, Sender, channel},
    },
    thread::spawn,
    time::SystemTime,
};

use simplelog::{Color, Config, ConfigBuilder, TermLogger};
use tdtp_impl::{
    client::{ChannelDataPacket, data},
    server::{OutgoingDataPacket, Server, ServerError},
};

fn main() -> Result<(), Box<dyn Error>> {
    TermLogger::init(
        log::LevelFilter::Debug,
        ConfigBuilder::new()
            .set_level_color(log::Level::Trace, Some(Color::Magenta))
            .set_target_level(log::LevelFilter::Error)
            .set_thread_level(log::LevelFilter::Error)
            .build(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )
    .unwrap();

    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let (client_tx, client_rx) = channel();
    let (server_tx, server_rx) = channel();

    produce_packages(server_tx);
    let consumer_thread = spawn(move || package_consumer(client_rx));
    let server_thread = spawn(move || Server::run(addr, 8000, server_rx));

    println!("a");
    data(addr, 8000, client_tx).expect("oh no");
    println!("b");
    consumer_thread.join().unwrap();
    println!("c");

    server_thread.join().unwrap().expect("oh no2")
}

fn package_consumer(rx: Receiver<ChannelDataPacket>) {
    let mut counter = 1;
    while counter <= 512
        && let Ok(packet) = rx.recv()
    {
        if let ChannelDataPacket::Packet(_) = packet {
            counter += 1;
        }
    }
}

fn produce_packages(tx: Sender<OutgoingDataPacket>) {
    for i in 1..=512 {
        // println!("{i}");
        tx.send(OutgoingDataPacket {
            time: SystemTime::now(),
        })
        .unwrap()
    }
}
