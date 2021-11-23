mod dns;
use dns::header::DNSHeader;

fn main(){
    let _listener = std::net::UdpSocket::bind("0.0.0.0:5514").unwrap();
    // listen and receive. 
    loop {
        let mut _buffer = [0u8; 1024];
        let (_count, _remoteaddr) = _listener.recv_from(&mut _buffer).unwrap();
        println!("Received {:?} from {}", &_buffer[0.._count], _remoteaddr);

        // Get a DNS Header outta that 
        let _newheader = DNSHeader::new(&_buffer[0.._count]);
        println!("DNS Header read as: {:?}", _newheader);
    }
}