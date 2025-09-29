//! Implementation of the TDT protocol, as defined in the specification in the root of this crate.
//!
//! ## Features
//!
//! - `client`: Enables client-side functions and data types
//! - `server`: Enables server-side functions and data types
//! - `full`: Enables all of the above
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
    ops::{Deref, DerefMut},
    sync::{
        atomic::AtomicBool,
        mpsc::{self},
        Arc,
    },
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

/// An MPSC receiver. Check the documentation of [`std::sync::mpsc::Receiver`] for more information.
pub struct Receiver<T>(mpsc::Receiver<T>, Arc<AtomicBool>);

impl<T> Deref for Receiver<T> {
    type Target = mpsc::Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Receiver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.1.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// An MPSC sender. Check the documentation of [`std::sync::mpsc::Sender`] for more information.
pub struct Sender<T>(mpsc::Sender<T>, Arc<AtomicBool>);

impl<T> Sender<T> {
    /// Check if this sender still
    pub(crate) fn has_receiver(&self) -> bool {
        !self.1.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl<T> Deref for Sender<T> {
    type Target = mpsc::Sender<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Sender<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Create an MPSC channel for receiving and sending data.
#[must_use]
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel();
    let tx = Sender(tx, Arc::new(AtomicBool::new(false)));
    let rx = Receiver(rx, tx.1.clone());

    (tx, rx)
}
