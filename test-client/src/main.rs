#![feature(thread_id_value)]

use std::{
    io::{self, Write as _},
    net::{IpAddr, Ipv4Addr},
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
    thread::{self, Builder, JoinHandle, sleep},
    time::{Duration, SystemTime},
};

use colored::Colorize;
use env_logger::fmt::Formatter;
use log::{Level, info, trace};
use tdtp_impl::{
    client::{ChannelDataPacket, data},
    server::{OutgoingDataPacket, Server},
};

fn color_level(f: &mut Formatter, lvl: Level) -> io::Result<()> {
    match lvl {
        Level::Error => write!(f, "{}", "[ERROR] ".red()),
        Level::Warn => write!(f, "{}", "[WARN] ".yellow()),
        Level::Info => write!(f, "{}", "[INFO] ".blue()),
        Level::Debug => write!(f, "{}", "[DEBUG] ".bright_cyan()),
        Level::Trace => write!(f, "{}", "[TRACE] ".purple()),
    }
}

fn init_logger() {
    env_logger::builder()
        .write_style(env_logger::WriteStyle::Always)
        .format(|buf, rec| {
            let curr_thread = thread::current();
            let (h, m, s, u) = time::UtcDateTime::now().time().as_hms_micro();

            write!(buf, "{h}:{m}:{s}.{u}us ")?;
            color_level(buf, rec.level())?;

            match curr_thread.name() {
                Some(v) => write!(buf, "({v}) "),
                None => write!(buf, "({}) ", curr_thread.id().as_u64()),
            }?;
            writeln!(buf, "{}: {}", rec.target(), rec.args())?;

            Ok(())
        })
        .filter_level(log::LevelFilter::Trace)
        .init();
}

fn named_thread<F, T>(name: &str, f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    Builder::new().name(name.to_string()).spawn(f).unwrap()
}

fn main() {
    init_logger();

    let (packet_producer_server, packet_supplier_server) = channel::<OutgoingDataPacket>();
    let packet_producer_server = Arc::new(packet_producer_server);
    let arc = Arc::clone(&packet_producer_server);
    let (packet_sender_client, packet_consumer_client) = channel::<ChannelDataPacket>();

    info!("Spawning packet producer thread");
    let _ = named_thread("Packet producer", move || {
        let mut ctr = 1;
        while ctr <= 128 {
            trace!("Producing {ctr}th packet");
            arc.send(OutgoingDataPacket {
                time: SystemTime::now(),
            })
            .unwrap();
            ctr += 1;
        }
    });

    info!("Spawning server thread");
    let _ = named_thread("Server", || {
        info!("Starting server");
        Server::run(
            (IpAddr::V4(Ipv4Addr::LOCALHOST), 8000),
            packet_supplier_server,
        )
    });

    // let both threads startup
    sleep(Duration::from_millis(100));

    info!("Spawning packet processor thread");
    let _ = named_thread("Packet processor", move || {
        processor(packet_consumer_client)
    });

    info!("Sending request");
    data(
        (IpAddr::V4(Ipv4Addr::LOCALHOST), 8000),
        packet_sender_client,
    )
    .unwrap();
}

fn processor(rx: Receiver<ChannelDataPacket>) {
    trace!("Entered packet processor thread");
    let mut ctr = 1;
    while ctr <= 128
        && let Ok(packet) = rx.recv()
    {
        trace!("Packet processor received {ctr}th packet: {packet:?}");
        ctr += 1;
    }

    trace!("Packet processor received 128 packets, exiting");
}
