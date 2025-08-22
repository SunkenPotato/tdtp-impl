use std::{
    io::{self, Write},
    net::TcpStream,
};

pub const CTRL: u8 = 0x11;
pub const NUL: u8 = 0x00;
pub const DATA_PACKET: u8 = !NUL;

pub const OP_EXIT: u8 = 0x19;
pub const OP_ERR: u8 = 0x18;
pub const OP_TRANS_END: u8 = 0x17;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ControlSignal {
    Exit = OP_EXIT,
    Err = OP_ERR,
    TransmissionEnd = OP_TRANS_END,
}

impl ControlSignal {
    pub fn write(self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&[CTRL, self as u8])
    }
}

pub const ERR_INVALID: u8 = 0x01;
pub const ERR_CONN_TY: u8 = 0x02;
pub const ERR_EOF: u8 = 0x03;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    Invalid = ERR_INVALID,
    UnknownConnection = ERR_CONN_TY,
    Eof = ERR_EOF,
}

impl Error {
    pub fn write(self, stream: &mut TcpStream) -> io::Result<()> {
        ControlSignal::Err.write(stream)?;
        stream.write_all(&[self as u8])
    }
}

pub const CONN_DATA: u8 = 0x01;
pub const CONN_STAT: u8 = 0x02;

#[repr(u8)]
pub enum ConnectionType {
    Data = CONN_DATA,
    Status = CONN_STAT,
}

pub const STAT_CONN_IP: u8 = 0b10000000;
pub const STAT_TIME: u8 = 0b01000000;
pub const STAT_ECHO: u8 = 0b00100000;
pub const STAT_VER: u8 = 0b00010000;
pub const STAT_5: u8 = 0b00001000;
pub const STAT_6: u8 = 0b00000100;
pub const STAT_7: u8 = 0b00000010;
pub const STAT_8: u8 = 0b00000001;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StatusDataFlag {
    ConnectionIp = STAT_CONN_IP,
    Time = STAT_TIME,
    Echo = STAT_ECHO,
    Version = STAT_VER,
    _Emp5 = STAT_5,
    _Emp6 = STAT_6,
    _Emp7 = STAT_7,
    _Emp8 = STAT_8,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StatusData(u8);

impl Default for StatusData {
    fn default() -> Self {
        Self(0b11110000)
    }
}

impl StatusData {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn data(&self) -> u8 {
        self.0
    }

    pub const fn with_flag(self, flag: StatusDataFlag) -> Self {
        Self(self.0 | (flag as u8))
    }

    pub const fn remove_flag(self, flag: StatusDataFlag) -> Self {
        Self(self.0 & !(flag as u8))
    }

    pub const fn has_flag(self, flag: StatusDataFlag) -> bool {
        (self.0 & (flag as u8)) == (flag as u8)
    }

    pub const unsafe fn from_u8(v: u8) -> Self {
        Self(v)
    }
}
