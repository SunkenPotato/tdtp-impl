use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use derive_more::From;

use crate::consts::{ConnectionType, ControlSignal, Error as ProtoError, StatusData, CTRL, NUL};

const ECHO_BUF_CAP: usize = 128;
const STATUS_OUT_MIN_CAP: usize = 4;

#[derive(Debug, From)]
pub enum ClientError {
    IoError(io::Error),
    ProtoError(ProtoError),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatusResponse {
    pub ip: Option<[u8; 4]>,
    pub time: Option<u128>,
    pub ver: Option<[u8; 3]>,
    pub echo: Option<Vec<u8>>,
}

pub fn status(
    flags: StatusData,
    addr: impl ToSocketAddrs,
    echo: Option<&[u8]>,
) -> Result<StatusResponse, ClientError> {
    let mut stream = TcpStream::connect(addr)?;

    let mut cap = STATUS_OUT_MIN_CAP;
    if flags.has_flag(crate::consts::StatusDataFlag::Echo) {
        cap += 1;
        cap += echo.unwrap().len();
    }

    let mut buffer = Vec::with_capacity(cap);
    buffer.push(ConnectionType::Status as u8);
    buffer.push(flags.data());

    if flags.has_flag(crate::consts::StatusDataFlag::Echo) {
        buffer.extend(echo.unwrap());
        buffer.push(NUL);
    }

    buffer.push(CTRL);
    buffer.push(ControlSignal::TransmissionEnd as u8);

    stream.write_all(&buffer)?;

    let mut reader = BufReader::new(stream);

    let mut ip = None;
    let mut time = None;
    let mut ver = None;
    let mut echo = None;

    if flags.has_flag(crate::consts::StatusDataFlag::ConnectionIp) {
        fill_option(&mut ip, &mut reader)?;
    }

    if flags.has_flag(crate::consts::StatusDataFlag::Time) {
        let mut tmp: Option<[u8; size_of::<u128>()]> = None;
        fill_option(&mut tmp, &mut reader)?;
        time = Some(u128::from_le_bytes(tmp.unwrap()));
    }

    if flags.has_flag(crate::consts::StatusDataFlag::Version) {
        fill_option(&mut ver, &mut reader)?;
    }

    if flags.has_flag(crate::consts::StatusDataFlag::Echo) {
        // we could make assumptions about the data, but if the protocol changes, it will require a rewrite
        let mut buf = Vec::with_capacity(ECHO_BUF_CAP);
        reader.read_until(NUL, &mut buf)?;
        buf.pop();
        echo = Some(buf);
    }

    let mut end = [0u8; 4];

    reader.read_exact(&mut end)?;

    if end
        != [
            CTRL,
            ControlSignal::TransmissionEnd as u8,
            CTRL,
            ControlSignal::Exit as u8,
        ]
    {
        return Err(ClientError::ProtoError(ProtoError::Invalid));
    }

    Ok(StatusResponse {
        ip,
        time,
        ver,
        echo,
    })
}

fn fill_option<const N: usize, R: Read>(
    opt: &mut Option<[u8; N]>,
    reader: &mut BufReader<R>,
) -> io::Result<()> {
    let mut tmp = [0; N];
    reader.read_exact(&mut tmp)?;
    *opt = Some(tmp);

    Ok(())
}
