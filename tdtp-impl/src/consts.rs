//! Constants for the protocol.

/// The control signal.
#[deprecated]
pub const CTRL: u8 = 0x11;
/// The empty signal, indicating that the server does not have a packet to send.
pub const EMP: u8 = 0x00;
/// The packet signal, indicating that a packet follows.
pub const SIG_PACKET: u8 = !EMP;

/// The connection data flag.
pub const CONN_DATA: u8 = 0x01;

/// Represents the different types of connections which are available.
// enum because we may add diff conn types in the future
#[repr(u8)]
pub enum ConnectionType {
    /// The data connection, for transmitting radioactivity data.
    Data = CONN_DATA,
}
