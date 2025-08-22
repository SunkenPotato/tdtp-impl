pub mod data;
pub mod status;

use std::{
    io::{self, Read},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
};

use derive_more::From;
use log::{debug, info};

use crate::{
    consts::{CONN_DATA, CONN_STAT, ConnectionType, ControlSignal, Error as ProtoError},
    pool::ThreadPool,
};
use data::data_handler;
use status::status_handler;

pub struct Server;

impl Server {
    pub fn run(ip: [u8; 4], port: u16, thread_n: usize) -> io::Result<!> {
        let pool = ThreadPool::new(thread_n);
        let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::from_octets(ip),
            port,
        )))?;

        while let Ok((conn, addr)) = listener.accept() {
            info!("Received connection from {addr}");

            pool.execute(move || match router(conn, addr) {
                Ok(_) => info!("Closed connection to {addr}"),
                Err(e) => {
                    info!("{addr} handler encoutered an I/O error: {e}");
                }
            });
        }

        unreachable!()
    }
}

fn close(mut stream: TcpStream) -> io::Result<()> {
    debug!("Writing exit signal to stream and closing");
    ControlSignal::Exit.write(&mut stream)?;
    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

#[derive(Debug, From)]
pub enum ServiceError {
    ProtoError(ProtoError),
    IoError(io::Error),
}

fn router(mut stream: TcpStream, addr: SocketAddr) -> io::Result<()> {
    let mut conn_ty = [0; 1];

    match stream.read(&mut conn_ty)? {
        1 => (),
        0 => {
            _ = ProtoError::Invalid.write(&mut stream);
            return Ok(());
        }
        _ => unreachable!(),
    }

    let connection_type = match conn_ty[0] {
        CONN_DATA => ConnectionType::Data,
        CONN_STAT => ConnectionType::Status,
        _ => {
            _ = ProtoError::UnknownConnection.write(&mut stream);
            return Ok(());
        }
    };

    let service = match connection_type {
        ConnectionType::Data => data_handler,
        ConnectionType::Status => status_handler,
    };

    match service(&mut stream, addr) {
        Ok(_) => {
            debug!("Writing transmission delimiter to connection");
            ControlSignal::TransmissionEnd.write(&mut stream)?;
        }
        Err(ServiceError::IoError(err)) => return Err(err),
        Err(ServiceError::ProtoError(err)) => {
            debug!("Service to {addr} returned a protocol error: {err:?}");
            err.write(&mut stream)?;
        }
    };

    close(stream)
}
