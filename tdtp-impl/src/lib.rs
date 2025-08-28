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

use std::{io, net::TcpStream};

use log::info;

#[cfg(feature = "client")]
pub mod client;
pub mod consts;
#[cfg(feature = "server")]
pub mod server;

fn close(stream: TcpStream) -> io::Result<()> {
    info!("Closing stream");
    stream.shutdown(std::net::Shutdown::Both)
}
