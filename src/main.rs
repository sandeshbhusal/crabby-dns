mod dns;
mod utils;
use dns::header::DNSHeader;

use crate::dns::question::DNSQuestion;

fn main(){
    let _listener = std::net::UdpSocket::bind("0.0.0.0:53").unwrap();
    // listen and receive. 
    loop {
        let mut _buffer = [0u8; 1024];
        let (_count, _remoteaddr) = _listener.recv_from(&mut _buffer).unwrap();
        // println!("Received {:?} from {}", &_buffer[0.._count], _remoteaddr);

        // Get a DNS Header outta that 
        let _newheader = DNSHeader::new(&_buffer[0.._count]).unwrap();
        println!("DNS Header read as: {:?}", _newheader);

        // Try to read DNS question as well :D
        let _newquestion = DNSQuestion::new(&_buffer[12.._count]).unwrap();
        println!("DNS Question read as: {:?}", _newquestion);
        println!("");
    }
}