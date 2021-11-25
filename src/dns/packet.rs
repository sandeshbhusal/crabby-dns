use super::{answer::DNSAnswer, header::DNSHeader, question::DNSQuestion};

/*
    Packet - encapsulates the header, query and answers into a coherent DNS packet.
    Exposes interfaces to directly read and write to byte streams.
*/
#[derive(Debug)]
struct DNSPacket {
    _header: DNSHeader,
    _questions: Vec<DNSQuestion>,
    _answers: Vec<DNSAnswer>
    // Authority and additional parts of the DNS packet have been skipped for now.
}

impl Default for DNSPacket{
    fn default() -> Self { 
        DNSPacket{
            _header: DNSHeader::default(),
            _questions: vec![DNSQuestion::default()],
            _answers: vec![DNSAnswer::default()]
        }
    }
}

impl DNSPacket{
    // Create a DNS packet from a slice of u8
    fn new(content: &[u8]) -> Result<Self, Box<dyn std::error::Error>>{
        let _content_vec = Vec::from(content);
        let mut _passable_iterator = _content_vec.iter();
        let _header_bytes = _passable_iterator.take(12);
        // let _header_bytes = _header_bytes.collect::<Vec<u8>>();
        Ok(DNSPacket::default())
    }
}