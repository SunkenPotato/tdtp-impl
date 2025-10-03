//! Client-side data types and functions.
//!
//! A connection can be established with the [`data`] function in this crate.

use std::{
    io::{self, BufReader, Read, Write},
    net::{IpAddr, TcpStream},
    sync::mpsc::SendError,
};

use log::{error, info, trace};

use crate::{
    client_mpsc, close,
    consts::{ConnectionType, EMP, SIG_EXIT, SIG_PACKET},
};

/// An incoming data packet, sent over a channel to be processed.
/// This must represent the amount of microseconds elapsed since the unix epoch.
pub type IncomingDataPacket = u128;

/// Initiate a data connection to the given address.
///
/// Once a packet is received, this function will send it to the other end of the supplied `supplier`.
///
/// If the receiver has hung up, this function will attempt to terminate the connection and exit with `Ok(())`.
///
/// The [`Sender`] requested by this function is not the [`std::sync::mpsc::Sender`]. It is a custom sender which allows this function to check
/// if the other side has hung up.
///
/// # Errors
/// May return an I/O error.
///
/// # Example
/// ```no_run
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
/// ```
#[expect(clippy::needless_pass_by_value)]
pub fn data(ip: IpAddr, port: u16, sender: client_mpsc::ClientSender) -> io::Result<()> {
    info!("Connecting to {ip}:{port}");
    let mut stream = TcpStream::connect((ip, port))?; // W
    info!("Connected to {ip}:{port}");
    let mut reader = BufReader::new(stream.try_clone()?); // R
    trace!("Sending data signal");
    stream.write_all(&[ConnectionType::Data as u8])?;

    let mut sig = [0xCE]; // some unused signal
    let mut data = [0; 16];

    loop {
        if !sender.has_receiver() {
            return Ok(());
        }

        trace!("Reading signal");
        reader
            .read_exact(&mut sig)
            .inspect_err(|v| error!("Failed to read signal: {v}"))?;

        match sig[0] {
            EMP => (),
            SIG_PACKET => {
                trace!("Reading data");
                reader.read_exact(&mut data)?;
                if handle_packet(data, &sender).is_err() {
                    trace!("Client packet receiver hung up, exiting");
                    break close(stream);
                }
            }
            SIG_EXIT => {
                info!("Server terminated connection, exiting");
                break Ok(());
            }
            _ => todo!(),
        }
    }
}

/// Convert the given bytes into an [`IncomingDataPacket`] and send them via the sender.
fn handle_packet(
    data: [u8; 16],
    sender: &client_mpsc::ClientSender,
) -> Result<(), SendError<IncomingDataPacket>> {
    sender.send(u128::from_le_bytes(data))
}

/// A C-compatible wrapper for [`data`].
///
/// # Safety
/// `sender` must be a valid pointer.
#[cfg(feature = "interop")]
#[expect(unsafe_code)]
#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn c_data(
    ip_a: u8,
    ip_b: u8,
    ip_c: u8,
    ip_d: u8,
    port: u16,
    sender: *mut (),
) -> i32 {
    use crate::client_mpsc::ClientSender;
    use std::net::Ipv4Addr;

    match data(
        IpAddr::V4(Ipv4Addr::new(ip_a, ip_b, ip_c, ip_d)),
        port,
        unsafe { *Box::from_raw(sender.cast::<ClientSender>()) },
    ) {
        Ok(()) => 0,
        Err(e) => e.raw_os_error().unwrap_or(-1),
    }
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
