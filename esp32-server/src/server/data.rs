use std::net::{SocketAddr, TcpStream};

use log::info;

use crate::server::ServiceError;

pub fn data_handler(_stream: &mut TcpStream, addr: SocketAddr) -> Result<(), ServiceError> {
    info!("Data connection with {addr} established");
    Ok(())
}
