use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{
        Arc,
        mpsc::{Sender, channel},
    },
    thread::spawn,
    time::{Duration, SystemTime},
};

use tdtp::server::{OutgoingDataPacket, server};

fn main() {
    let (tx, rx) = channel();
    let arc = Arc::new(tx);
    // we use an Arc here because if tx is dropped while the server is running, it will exit with Err(ServerError::ConnectionTermination),
    // which may not be something you want. an Arc will keep it alive for the duration of `main`.
    let _producer_thread = spawn(move || package_producer(Arc::clone(&arc)));

    server(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000, rx).expect("oops, server error");
}

fn package_producer(tx: Arc<Sender<OutgoingDataPacket>>) {
    // in your case, you may only want to send a packet when a certain event occurs,
    // such as when a particle is detected or every 5 seconds. you may also only want to send a certain amount of packets.

    for _ in 0..512 {
        tx.send(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        )
        .unwrap();

        std::thread::sleep(Duration::from_secs(5));
    }
}
