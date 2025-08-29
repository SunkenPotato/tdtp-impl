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
#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![forbid(clippy::missing_docs_in_private_items)]

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
