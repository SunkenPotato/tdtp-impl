use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpStream},
};

use log::{debug, info};

use crate::{
    VERSION, VersionUnion,
    consts::{CTRL, ControlSignal, Error as ProtoError, NUL, StatusData, StatusDataFlag},
    server::ServiceError,
    time_bytes,
};

pub fn status_handler(stream: &mut TcpStream, addr: SocketAddr) -> Result<(), ServiceError> {
    info!("Status connection with {addr} established");

    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    let mut out = Vec::with_capacity(512);

    let mut flag = [0];

    match reader.read(&mut flag)? {
        1 => (),
        0 => return Err(ServiceError::ProtoError(ProtoError::Eof)),
        _ => unreachable!(),
    }

    // SAFETY: the from_u8 function is currently unsafe but has no requirements
    let flags = unsafe { StatusData::from_u8(flag[0]) };

    if flags.has_flag(StatusDataFlag::ConnectionIp) {
        debug!("{addr} has status flag STAT_CONN_IP, sending back IP");
        out.extend(addr.ip().as_octets());
    }

    match flags.has_flag(StatusDataFlag::Time) {
        true => {
            debug!("{addr} has status flag STAT_TIME, sending back time");
            out.extend(time_bytes());
        }
        false => (),
    }

    if flags.has_flag(StatusDataFlag::Version) {
        debug!("{addr} has status flag STAT_VER, sending back version");
        out.extend(unsafe { VersionUnion { ver: *VERSION }.bytes });
    }

    if flags.has_flag(StatusDataFlag::Echo) {
        debug!("{addr} has status flag STAT_ECHO");

        let mut buf = Vec::with_capacity(256);

        reader.read_until(NUL, &mut buf)?;

        debug!("{addr} sent echo data: {buf:?}");

        out.extend(buf);
    }

    debug!("{addr} trying to get transmission delim");

    let mut buf = [0; 2];

    debug!("{addr} calling stream.read");

    match reader.read(&mut buf)? {
        2 => debug!("{addr} OK.. got 2 bytes"),
        n @ ..2 => {
            debug!("{addr} EOF, got {n} bytes");
            return Err(ServiceError::ProtoError(ProtoError::Eof));
        }
        _ => unreachable!(),
    }

    if buf != [CTRL, ControlSignal::TransmissionEnd as u8] {
        return Err(ServiceError::ProtoError(ProtoError::Eof));
    }

    debug!("Got transmission delimiters, ending status connection");

    writer.write_all(&out)?;

    Ok(())
}
