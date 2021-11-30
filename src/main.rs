mod dns;
mod utils;
use dns::{header::DNSHeader, packet::DNSPacket};

use crate::dns::question::DNSQuestion;

fn main(){
    let _listener = std::net::UdpSocket::bind("0.0.0.0:53").unwrap();
    // listen and receive. 
    let mut _buffer = [0u8; 512];
    loop {
        let (_count, _remoteaddr) = _listener.recv_from(&mut _buffer).unwrap();
        let _packet = DNSPacket::new(&_buffer[.._count]);
        _buffer.iter_mut().for_each(|x| *x = 0); // Zero out the buffer.
    }
}