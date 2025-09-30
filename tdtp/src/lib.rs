//! Implementation of the TDT protocol, as defined in the specification in the root of this crate.
//!
//! ## Features
//!
//! + `client`: Enables client-side functions and data types
//! + `server`: Enables server-side functions and data types
//! + `interop`: Enables interoperability interfaces for C/C++ code.
//! + `full`: Enables all of the above
//!
//! View the module-level docs for more information on usage.

#![feature(never_type)]
#![forbid(missing_docs)]
#![cfg_attr(not(feature = "interop"), forbid(unsafe_code))]
// Relax it for external APIs
#![cfg_attr(feature = "interop", deny(unsafe_code))]
#![forbid(clippy::allow_attributes)]
#![forbid(clippy::missing_docs_in_private_items)]
#![forbid(unfulfilled_lint_expectations)]
#![deny(clippy::pedantic)]

use std::{
    io::{self, Write as _},
    net::TcpStream,
};

use log::info;

use crate::consts::SIG_EXIT;

#[cfg(feature = "client")]
pub mod client;
pub mod consts;
#[cfg(feature = "server")]
pub mod server;

/// Close the given TCP stream by shutting down both R/W sides.
fn close(mut stream: TcpStream) -> io::Result<()> {
    info!("Closing stream");
    stream.write_all(&[SIG_EXIT])?;
    stream.shutdown(std::net::Shutdown::Both)
}

/// A channel pair.
#[repr(C)]
#[cfg(feature = "interop")]
pub struct ChannelPair {
    /// The sender.
    pub tx: *mut (),
    /// The receiver.
    pub rx: *mut (),
}

/// Initialise a logging framework. This is meant for external callers who cannot instantiate a Rust logging framework.
#[cfg(feature = "interop")]
#[expect(clippy::missing_panics_doc)]
#[expect(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn init_logger_framework() {
    use simplelog::{Config, SimpleLogger};

    SimpleLogger::init(log::LevelFilter::Debug, Config::default()).unwrap();
}

pub mod client_mpsc {
    //! MPSC channels for client side of the protocol. These differ from normal channel ([`std::sync::mpsc`]) in the way that the sender keeps track of whether an associated receiver exists.

    use std::{
        ops::{Deref, DerefMut},
        sync::{Arc, atomic::AtomicBool, mpsc},
    };

    use crate::client::IncomingDataPacket;

    /// Type alias for the inner receiver of [`ClientReceiver`].
    ///
    /// cbindgen:ignore
    type InnerRecv = mpsc::Receiver<IncomingDataPacket>;
    /// Type alias for the inner sender of [`ClientSender`].
    ///
    /// cbindgen:ignore
    type InnerSender = mpsc::SyncSender<IncomingDataPacket>;

    /// An MPSC receiver. Check the documentation of [`std::sync::mpsc::Receiver`] for more information.
    #[repr(C)]
    pub struct ClientReceiver {
        /// The inner [`std::sync::mpsc::Receiver`].
        inner: InnerRecv,
        /// Whether this receiver has been dropped. This is updated in [`ClientReceiver::drop`].
        drop_flag: Arc<AtomicBool>,
    }

    impl Deref for ClientReceiver {
        type Target = InnerRecv;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for ClientReceiver {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    impl Drop for ClientReceiver {
        fn drop(&mut self) {
            self.drop_flag
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// An MPSC sender. Check the documentation of [`std::sync::mpsc::Sender`] for more information.
    #[repr(C)]
    pub struct ClientSender {
        /// The inner [`std::sync::mpsc::Sender`].
        inner: InnerSender,
        /// Whether the receiver has been dropped.
        drop_flag: Arc<AtomicBool>,
    }

    impl ClientSender {
        /// Check if this sender still has an associated receiver.
        pub(crate) fn has_receiver(&self) -> bool {
            !self.drop_flag.load(std::sync::atomic::Ordering::Relaxed)
        }

        /// A C-compatible wrapper around [`Self::has_receiver`].
        #[cfg(feature = "interop")]
        #[must_use]
        pub extern "C" fn c_has_receiver(&self) -> bool {
            self.has_receiver()
        }
    }

    impl Deref for ClientSender {
        type Target = InnerSender;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for ClientSender {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    /// Create an MPSC channel for receiving and sending data.
    #[must_use]
    pub fn client_channel(buffer: usize) -> (ClientSender, ClientReceiver) {
        let (tx, rx) = mpsc::sync_channel(buffer);
        let tx = ClientSender {
            inner: tx,
            drop_flag: Arc::new(AtomicBool::new(false)),
        };
        let rx = ClientReceiver {
            inner: rx,
            drop_flag: Arc::clone(&tx.drop_flag),
        };

        (tx, rx)
    }

    /// C-compatible wrapper for [`client_channel`]. This returns a `*mut ChannelPair<T>` because `ChannelPair<T>` is not FFI-safe.
    ///
    /// # Safety
    /// The receiver must be freed correctly with the [`c_free_receiver`] function.
    #[cfg(feature = "interop")]
    #[expect(unsafe_code)]
    #[unsafe(no_mangle)]
    #[must_use]
    pub unsafe extern "C" fn c_client_channel(buffer: usize) -> crate::ChannelPair {
        let (tx, rx) = client_channel(buffer);
        let tx = Box::into_raw(Box::new(tx)).cast::<()>();
        let rx = Box::into_raw(Box::new(rx)).cast::<()>();

        crate::ChannelPair { tx, rx }
    }

    /// Safely drop the passed receiver.
    ///
    /// # Safety
    /// `recv` must be a valid pointer.
    #[cfg(feature = "interop")]
    #[expect(unsafe_code)]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn c_free_client_receiver(recv: *mut ()) {
        drop(unsafe { Box::from_raw(recv.cast::<ClientReceiver>()) });
    }

    /// Receive an incoming data packet from the given receiver. If the sender has hung up, this return `false`, else `true`.
    ///
    /// This will block. For a non-blocking alternative, see `c_client_channel_try_recv`.
    ///
    /// # Safety
    /// `receiver` and `out` must be valid pointers.
    #[cfg(feature = "interop")]
    #[expect(unsafe_code)]
    #[unsafe(no_mangle)]
    #[must_use]
    pub unsafe extern "C" fn c_client_channel_recv(
        out: *mut IncomingDataPacket,
        receiver: *mut (),
    ) -> bool {
        let receiver = unsafe { &*receiver.cast::<ClientReceiver>() };
        match receiver.recv() {
            Ok(v) => unsafe {
                *out = v;
                true
            },
            Err(_) => false,
        }
    }

    /// Receive an incoming data packet from the given receiver.
    ///
    /// If the sender has hung up,
    /// this returns `1`. If there are no packets to be received, this returns `2`. If a packet was
    /// successfully received, this returns `0`.
    ///
    /// # Safety
    /// `receiver` and `out` must be valid pointers.
    #[cfg(feature = "interop")]
    #[expect(unsafe_code)]
    #[unsafe(no_mangle)]
    #[must_use]
    pub unsafe extern "C" fn c_client_channel_try_recv(
        out: *mut IncomingDataPacket,
        receiver: *mut (),
    ) -> i32 {
        use std::sync::mpsc::TryRecvError;

        let receiver = unsafe { &*receiver.cast::<ClientReceiver>() };
        match receiver.try_recv() {
            Ok(v) => unsafe {
                *out = v;
                0
            },
            Err(TryRecvError::Disconnected) => 1,
            Err(TryRecvError::Empty) => 2,
        }
    }
}
