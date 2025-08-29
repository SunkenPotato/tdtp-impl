//! Server-side functions and data types.
//!
//! To instantiate a server, see [`Server::run`].

mod data;

use std::{
    io::{self, Read},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::mpsc::Receiver,
    time::{SystemTime, UNIX_EPOCH},
};

use log::{debug, error, info};

use crate::{
    close,
    consts::{CONN_DATA, ConnectionType},
    server::data::data_handler,
};

/// An outgoing data packet, i.e., one which the server intends to send.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct OutgoingDataPacket {
    /// The system time associated with this packet.
    pub time: SystemTime,
}

impl OutgoingDataPacket {
    /// Converts `self` into a little-endian encoded `u128` integer, which represents the microseconds since the unix epoch.
    fn as_bytes(self) -> [u8; 16] {
        self.time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
            .to_le_bytes()
    }
}

/// A server.
///
/// View [`Server::run`] on how to use this.
pub struct Server;

impl Server {
    /// Listen for a connection at the given address.
    ///
    /// The server will relay the packets sent over the given `supplier` to the connector.
    ///
    /// Note: this is a single-threaded server, it does not support multiple simultaneous connections.
    ///
    /// # Examples
    /// ```no_run
    /// use std::{thread::spawn, sync::mpsc};
    /// use core::net::{IpAddr, Ipv4Addr};
    /// use tdtp_impl::server::Server;
    ///
    /// let (tx, rx) = mpsc::channel();
    ///
    /// let supplier_thread = spawn(move || loop {
    ///     tx.send(todo!()); // send a packet to the server
    /// });
    ///
    /// Server::run(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000, rx).expect("an I/O error occurred");
    /// ```
    pub fn run(ip: IpAddr, port: u16, supplier: Receiver<OutgoingDataPacket>) -> io::Result<!> {
        info!("Starting listener");
        let listener = TcpListener::bind((ip, port))?;

        info!(
            "Started listener at {}:{}, now listening for connections",
            ip, port
        );

        while let Ok((conn, addr)) = listener.accept() {
            info!("Received connection from {addr}");

            match router(conn, addr, &supplier) {
                Ok(_) => info!("Closed connection to {addr}"),
                Err(e) => {
                    error!("{addr} handler encoutered an I/O error: {e}");
                }
            }
        }

        unreachable!()
    }
}

/// Route the incoming connection to a handler.
///
/// Currently, this has only one handler registered.
fn router(
    mut stream: TcpStream,
    addr: SocketAddr,
    supplier: &Receiver<OutgoingDataPacket>,
) -> io::Result<()> {
    let mut conn_ty = [0; 1];

    stream.read_exact(&mut conn_ty)?;

    match conn_ty[0] {
        CONN_DATA => ConnectionType::Data,
        _ => {
            return Ok(());
        }
    };

    match data_handler(&mut stream, addr, supplier) {
        Ok(_) => {
            debug!("Writing transmission delimiter to connection");
        }
        Err(e) => {
            error!("Service encountered an I/O error: {e}");
        }
    };

    close(stream)
}
