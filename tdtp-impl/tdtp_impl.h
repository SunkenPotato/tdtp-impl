#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// The control signal.
constexpr static const uint8_t CTRL = 17;

/// The empty signal, indicating that the server does not have a packet to send.
constexpr static const uint8_t EMP = 0;

/// The packet signal, indicating that a packet follows.
constexpr static const uint8_t SIG_PACKET = ~EMP;

/// The exit signal.
constexpr static const uint8_t SIG_EXIT = 25;

/// The connection data flag.
constexpr static const uint8_t CONN_DATA = 1;

/// An MPSC receiver. Check the documentation of [`std::sync::mpsc::Receiver`] for more information.
template<typename T = void>
struct Receiver;

/// An MPSC sender. Check the documentation of [`std::sync::mpsc::Sender`] for more information.
template<typename T = void>
struct Sender;

/// An incoming data packet, sent over a channel to be processed.
using IncomingDataPacket = u128;

/// An outgoing data packet, i.e., one which the server intends to send.
using OutgoingDataPacket = SystemTime;

extern "C" {

/// A C-compatible wrapper for [`data`].
///
/// # Safety
/// `sender` must be a valid pointer.
int32_t c_data(uint8_t ip_a,
               uint8_t ip_b,
               uint8_t ip_c,
               uint8_t ip_d,
               uint16_t port,
               const Sender<IncomingDataPacket> *sender);

/// A C-compatible wrapper around [`Server::run`].
///
/// If `-1` is returned, the I/O error returned by [`Server::run`] was not constructed via
/// `last_os_error` or `from_raw_os_error`.
///
/// If `-2` is returned, the channel was closed while the server was running.
///
/// # Safety
/// `receiver` must be a valid pointer.
int32_t c_server(uint8_t ip_a,
                 uint8_t ip_b,
                 uint8_t ip_c,
                 uint8_t ip_d,
                 uint16_t port,
                 const Receiver<OutgoingDataPacket> *receiver);

}  // extern "C"
