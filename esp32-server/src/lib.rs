#![feature(ip_as_octets)]
#![feature(ip_from)]
#![feature(never_type)]

pub mod client;
pub mod consts;
pub mod pool;
pub mod server;

use std::{
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Version {
    maj: u8,
    min: u8,
    patch: u8,
}

union VersionUnion {
    ver: Version,
    bytes: [u8; 3],
}

impl Version {
    pub fn get() -> Self {
        let maj = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        let min = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

        Self { maj, min, patch }
    }
}

pub static VERSION: LazyLock<Version> = LazyLock::new(Version::get);

fn time_bytes() -> [u8; size_of::<u128>()] {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_le_bytes()
}
