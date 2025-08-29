use std::{
    net::{IpAddr, Ipv4Addr},
    thread::spawn,
};

use tdtp_impl::{
    Receiver, channel,
    client::{IncomingDataPacket, data},
};

fn main() {
    let (tx, rx) = channel();
    let _consumer_thread = spawn(|| {
        package_consumer(rx);
    });

    // once `rx` is dropped, data will end the connection
    data(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000, tx).expect("oops, I/O error");
}

fn package_consumer(receiver: Receiver<IncomingDataPacket>) {
    let mut counter = 1;

    while counter <= 512
        && let Ok(packet) = receiver.recv()
    {
        // we match on ChannelDataPacket::Packet because it may also be a ChannelDataPacket::__Ping packet,
        // which should not be handled
        println!("Got a packet: {packet:?}");
        counter += 1;
    }
}
