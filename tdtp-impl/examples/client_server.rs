use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr},
    sync::mpsc::{self, Sender},
    thread::spawn,
    time::SystemTime,
};

use tdtp_impl::{
    channel,
    client::{data, IncomingDataPacket},
    server::{OutgoingDataPacket, Server},
    Receiver,
};

fn main() -> Result<(), Box<dyn Error>> {
    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    // create a channel for the client and the package consumer to communicate
    let (client_tx, client_rx) = channel();
    // create a channel for the server and the package producer to communicate
    let (server_tx, server_rx) = mpsc::channel();

    // we're not going to actually use a thread here to constantly generate packages, instead
    // we'll produce them all before we start the server. since channels are buffered, they'll stay in memory until the server consumes them.
    produce_packages(server_tx);
    // this thread will receive packages from the client...
    let consumer_thread = spawn(move || package_consumer(client_rx));
    // and this one will be our server thread, running at 127.0.0.1:8000
    let server_thread = spawn(move || Server::run(addr, 8000, server_rx));

    // initiate the connection
    data(addr, 8000, client_tx).expect("oh no, client error");

    // and then wait for the threads to finish
    consumer_thread.join().unwrap();
    server_thread.join().unwrap().expect("oh no, server error")
}

// this will consume 512 packages and then exit, which will drop `rx`.
// once `rx` is dropped, the client will terminate the connection with the server and return.
fn package_consumer(rx: Receiver<IncomingDataPacket>) {
    let mut zähler = 1;
    while zähler <= 512
        && let Ok(packet) = rx.recv()
    {
        println!("Got packet: {packet:?}");
        zähler += 1;
    }
}

/// produce 512 packages, each with the current time. even though `tx` is dropped here, since channels are buffered,
/// when the server calls `Receiver::recv` on its end, it will still receive packages, regardless of the other side having hung up.
fn produce_packages(tx: Sender<OutgoingDataPacket>) {
    for _ in 1..=512 {
        tx.send(OutgoingDataPacket {
            time: SystemTime::now(),
        })
        .unwrap()
    }
}
