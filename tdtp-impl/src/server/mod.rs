//! Server-side functions and data types.
//!
//! To instantiate a server, see [`Server::run`].

use std::{
    fmt::Display,
    io::{self, BufReader, ErrorKind, Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{Receiver, TryRecvError},
    time::{SystemTime, UNIX_EPOCH},
};

use log::{debug, error, info, warn};

use crate::{
    close,
    consts::{CONN_DATA, CTRL, ConnectionType, EMP, SIG_EXIT, SIG_PACKET},
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

/// A server error.
#[derive(Debug)]
pub enum ServerError {
    /// An I/O error was encountered.
    IoError(io::Error),
    /// The supplier hung up.
    ChannelTermination,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{e}"),
            Self::ChannelTermination => write!(f, "Channel disconnected"),
        }
    }
}

impl From<io::Error> for ServerError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

/// Listen for a connection at the given address.
///
/// The server will relay the packets sent over the given `supplier` to the connector.
/// If `supplier` hangs up, the server will exit with `Err(ServerError::ChannelTermination)`.
///
/// Note: this is a single-threaded server, it does not support multiple simultaneous connections.
///
/// # Errors
/// Returns either an I/O error or an error indicating that the receiver to the supplied sender hung up.
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
pub fn server(
    ip: IpAddr,
    port: u16,
    supplier: &Receiver<OutgoingDataPacket>,
) -> Result<!, ServerError> {
    info!("Starting listener");
    let listener = TcpListener::bind((ip, port))?;

    info!("Started listener at {ip}:{port}, now listening for connections",);

    while let Ok((conn, addr)) = listener.accept() {
        info!("Received connection from {addr}");

        match router(conn, addr, supplier) {
            Ok(()) => info!("Closed connection to {addr}"),
            Err(e @ ServerError::ChannelTermination) => return Err(e),
            Err(e) => {
                error!("{addr} handler encoutered an error: {e}");
                return Err(e);
            }
        }
    }

    unreachable!()
}

/// Route the incoming connection to a handler.
///
/// Currently, this has only one handler registered.
fn router(
    mut stream: TcpStream,
    addr: SocketAddr,
    supplier: &Receiver<OutgoingDataPacket>,
) -> Result<(), ServerError> {
    let mut conn_ty = [0; 1];

    stream.read_exact(&mut conn_ty)?;

    match conn_ty[0] {
        CONN_DATA => ConnectionType::Data,
        _ => {
            return Ok(());
        }
    };

    match data_handler(&mut stream, addr, supplier) {
        Ok(()) => {
            debug!("Writing transmission delimiter to connection");
        }
        Err(e @ ServerError::ChannelTermination) => {
            close(stream)?;
            return Err(e);
        }
        Err(e @ ServerError::IoError(_)) => {
            error!("Service encountered an I/O error: {e}");
            return Err(e);
        }
    }

    Ok(close(stream)?)
}

/// The handler for the [`CONN_DATA`] connection.
pub(crate) fn data_handler(
    stream: &mut TcpStream,
    addr: SocketAddr,
    supplier: &Receiver<OutgoingDataPacket>,
) -> Result<(), ServerError> {
    info!("Data connection with {addr} established");

    // we do not want to block, since the client may not send anything at all (see find_exit_sig)
    stream.set_nonblocking(true)?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(&*stream);

    loop {
        if find_exit_sig(&mut reader)? {
            info!("Client sent exit signal, disconnecting");
            break Ok(());
        }

        let packet = match supplier.try_recv() {
            Err(TryRecvError::Disconnected) => {
                warn!("Data packet supplier hung up, terminating connection with client");
                break Err(ServerError::ChannelTermination);
            }
            Err(TryRecvError::Empty) => {
                write_nothing(&mut writer)?;
                continue;
            }
            Ok(v) => v,
        };

        write_packet(packet, &mut writer)?;
    }
}

/// Try to read an exit signal from the given stream. If a signal is found, this will return `Ok(true)`.
/// If not, it will return `Ok(false)`.
fn find_exit_sig(reader: &mut impl Read) -> io::Result<bool> {
    let mut buf = [0; 2];
    match reader.read(&mut buf) {
        Ok(2) if buf == [CTRL, SIG_EXIT] => Ok(true),
        Ok(_) => Ok(false),
        Err(e) if e.kind() == ErrorKind::WouldBlock => Ok(false),
        Err(e) => Err(e),
    }
}

/// Write the [`EMP`] byte to this sink. Convenience function.
fn write_nothing(sink: &mut impl Write) -> io::Result<()> {
    sink.write_all(&[EMP])
}

/// Write the given packet into this sink.
fn write_packet(packet: OutgoingDataPacket, sink: &mut impl Write) -> io::Result<()> {
    let mut data = [SIG_PACKET; 17];
    let bytes = packet.as_bytes();
    data[1..].copy_from_slice(&bytes);
    sink.write_all(&data)
}

/// A C-compatible wrapper around [`Server::run`].
///
/// If `-1` is returned, the I/O error returned by [`Server::run`] was not constructed via
/// `last_os_error` or `from_raw_os_error`.
///
/// If `-2` is returned, the channel was closed while the server was running.
///
/// # Safety
/// `receiver` must be a valid pointer.
#[cfg(feature = "interop")]
#[expect(unsafe_code)]
#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn c_server(
    ip_a: u8,
    ip_b: u8,
    ip_c: u8,
    ip_d: u8,
    port: u16,
    receiver: *const Receiver<OutgoingDataPacket>,
) -> i32 {
    use std::net::Ipv4Addr;

    match server(
        IpAddr::V4(Ipv4Addr::new(ip_a, ip_b, ip_c, ip_d)),
        port,
        unsafe { &*receiver },
    ) {
        Ok(_) | Err(ServerError::ChannelTermination) => 0,

        Err(ServerError::IoError(io)) => io.raw_os_error().unwrap_or(-1),
    }
}
