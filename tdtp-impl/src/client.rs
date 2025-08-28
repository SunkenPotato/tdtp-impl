//! Client-side data types and functions.
//!
//! A connection can be established with the [`data`] function in this crate.

use std::{
    io::{self, BufReader, Read, Write},
    net::{IpAddr, TcpStream},
    sync::mpsc::Sender,
};

use log::{info, trace};

use crate::{
    close,
    consts::{ConnectionType, EMP, SIG_PACKET},
};

/// An incoming data packet, sent over a channel to be processed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct IncomingDataPacket {
    /// The microseconds elapsed since the unix epoch when this was measured.
    pub time: u128,
}

/// A channel data packet.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChannelDataPacket {
    /// A ping packet.
    ///
    /// This exists to check whether the other side (the receiver) has hung up,
    /// and therefore the client should disconnect from the server. If it is received, it should be ignored.
    Ping,
    /// An actual packet.
    Packet(IncomingDataPacket),
}

/// Initiate a data connection to the given address.
///
/// Once a packet is received, this function will send it to the other end of the supplied `supplier`.
///
/// If the receiver has hung up, this function will attempt to terminate the connection and exit with `Ok(())`.
///
/// # Example
/// ```rs
/// use tdtp_impl::client::{ChannelDataPacket, data};
/// use core::net::{IpAddr, Ipv4Addr};
/// use std::{thread::spawn, sync::mpsc::channel};
///
/// let (tx, rx) = channel();
///
/// let consumer_thread = spawn(move || {
///     while let Ok(packet) = rx.recv() {
///         if let ChannelDataPacket::Packet(packet) = packet {
///             println!("Got a packet: {packet:?}");
///         }
///     }
/// });
///
/// data(
///     IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
///     8000,
///     tx
/// );
pub fn data(ip: IpAddr, port: u16, sender: Sender<ChannelDataPacket>) -> io::Result<()> {
    static TARGET: &str = "data_connection_client";

    info!(target: TARGET, "Connecting to {}:{}", ip, port);
    let mut stream = TcpStream::connect((ip, port))?; // W
    info!(target: TARGET, "Connected to {}:{}", ip, port);
    let mut reader = BufReader::new(stream.try_clone()?); // R
    trace!(target: TARGET, "Sending data signal");
    stream.write_all(&[ConnectionType::Data as u8])?;

    let mut sig = [0xCE]; // some unused signal
    let mut data = [0; 16];
    let mut ctr = 1;

    loop {
        reader.read_exact(&mut sig)?;
        if sender.send(ChannelDataPacket::Ping).is_err() {
            trace!("Client packet receiver hung up, exiting");
            break Ok(());
        }

        match sig[0] {
            EMP => continue,
            SIG_PACKET => {
                reader.read_exact(&mut data)?;
                if handle_packet(data, &sender) {
                    trace!("Client packet receiver hung up, exiting");
                    break close(stream);
                }
                trace!("Client received {ctr}th packet");
                ctr += 1;
            }
            _ => todo!(),
        }
    }
}

fn handle_packet(data: [u8; 16], sender: &Sender<ChannelDataPacket>) -> bool {
    sender
        .send(ChannelDataPacket::Packet(IncomingDataPacket {
            time: u128::from_le_bytes(data),
        }))
        .is_err()
}

// synchronisation:
// what is our problem?
// SystemTime may be too inaccurate, so we Instant instead, which, however,
// is relative.
// the packets sent will have a timestamp relative to one that both
// server and client must agree on, a sort of "ground zero".
// what do i want to do?
// synchronise this "ground zero" timestamp between client and server

// how do we do this?
// 1. the server will send some sort of signal (0xAA as an ex) (S1).
// once the client receives it, it should create an Instant at now (Instant A_C)
// once the server has sent it, it should also create an Instant at now (Instant A_S)

// 2. the client will send back a signal (0xAA) (S2)
// once the client has sent it, it should create a duration measuring how long it
// has been since A_C (D_C)
// once the server has received it, it should create a duration measuring how long it
// has been since A_S (D_S)

// 3. server should send D_S to client
// client should calc D_S - D_C and apply it to A_C, so:
// A_C += D_S - D_C

// diagram?
//    C        S
//    |Signal1 |
// A_C|--->----|A_S
//    |Signal2 |
// B_C|---<----|B_S
//    | D      |
//    |---<----|
