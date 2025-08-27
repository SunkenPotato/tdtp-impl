use std::{
    io::{self, BufReader, ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream},
    sync::mpsc::{Receiver, TryRecvError},
};

use log::{info, trace, warn};

use crate::{
    consts::{CTRL, EMP, SIG_EXIT, SIG_PACKET},
    server::OutgoingDataPacket,
};

pub(crate) fn data_handler(
    stream: &mut TcpStream,
    addr: SocketAddr,
    supplier: &Receiver<OutgoingDataPacket>,
) -> io::Result<()> {
    info!("Data connection with {addr} established");

    // we do not want to block, since the client may not send anything at all (see find_exit_sig)
    stream.set_nonblocking(true)?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(&*stream);

    let mut ctr = 1;

    loop {
        if find_exit_sig(&mut reader)? {
            info!("Client sent exit signal, disconnecting");
            break Ok(());
        }

        let packet = match supplier.try_recv() {
            Err(TryRecvError::Disconnected) => {
                warn!("Data packet supplier hung up, terminating connection with client");
                break Ok(());
            }
            Err(TryRecvError::Empty) => {
                write_nothing(&mut writer)?;
                continue;
            }
            Ok(v) => v,
        };

        write_packet(packet, &mut writer)?;
        trace!("Sent {ctr}th packet");
        ctr += 1;
    }
}

fn find_exit_sig(reader: &mut impl Read) -> io::Result<bool> {
    let mut buf = [0; 2];
    match reader.read(&mut buf) {
        Ok(2) if buf == [CTRL, SIG_EXIT] => Ok(true),
        Ok(_) => Ok(false),
        Err(e) if e.kind() == ErrorKind::WouldBlock => Ok(false),
        Err(e) => Err(e),
    }
}

fn write_nothing(stream: &mut impl Write) -> io::Result<()> {
    stream.write_all(&[EMP])
}

fn write_packet(packet: OutgoingDataPacket, sink: &mut impl Write) -> io::Result<()> {
    let mut data = [SIG_PACKET; 17];
    let bytes = packet.as_bytes();
    data[1..].copy_from_slice(&bytes);
    sink.write_all(&data)
}
