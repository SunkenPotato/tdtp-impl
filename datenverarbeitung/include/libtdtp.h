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


/// A channel pair.
struct ChannelPair {
  /// The sender.
  void *tx;
  /// The receiver.
  void *rx;
};

/// A packet to be sent, representing the amount of microseconds since the unix epoch.
using OutgoingDataPacket = unsigned long long int;
/// A received packet, representing the amount of microseconds since the unix epoch.
using IncomingDataPacket = unsigned long long int;

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
               void *sender);

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
                 void *receiver);

/// C-compatible wrapper for [`client_channel`]. This returns a `*mut ChannelPair` because `ChannelPair` is not FFI-safe.
///
/// # Safety
/// The receiver must be freed correctly with the [`c_free_receiver`] function.
ChannelPair c_client_channel(size_t buffer);

/// Create an MPSC channel for the server.
///
/// # Safety
/// The sender must be correctly disposed of with [`c_free_server_sender`].
ChannelPair c_server_channel(size_t buffer);

/// Safely drop the passed receiver.
///
/// # Safety
/// `recv` must be a valid pointer.
void c_free_client_receiver(void *recv);

/// Safely drop the passed sender.
///
/// # Safety
/// `sender` must be a valid pointer.
void c_free_server_sender(void *sender);

/// Send the given packet over the supplied sender.
///
/// # Safety
/// `sender` must be a valid pointer.

/// Send the given packet over the supplied server sender.
///
/// # Safety
/// `sender` must be a valid pointer.
///
bool c_server_channel_send(OutgoingDataPacket packet, const void *sender);

/// Receive an incoming data packet from the given receiver. If the sender has hung up, this return `false`, else `true`.
///
/// This will block. For a non-blocking alternative, see `c_client_channel_try_recv`.
///
/// # Safety
/// `receiver` and `out` must be valid pointers.
bool c_client_channel_recv(IncomingDataPacket *out, const void *receiver);

/// Receive an incoming data packet from the given receiver.
///
/// If the sender has hung up,
/// this returns `1`. If there are no packets to be received, this returns `2`. If a packet was
/// successfully received, this returns `0`.
///
/// # Safety
/// `receiver` and `out` must be valid pointers.
int32_t c_client_channel_try_recv(IncomingDataPacket *out, const void *receiver);

/// Initialise a logging framework. This is meant for external callers who cannot instantiate a Rust logging framework.
///
/// `filter` must be one of, where each level is more verbose than the preceding one:
/// - `0`: off
/// - `1`: error
/// - `2`: warn
/// - `3`: info
/// - `4`: debug
/// - `5`: trace
bool init_logger_framework(size_t filter);

}  // extern "C"
