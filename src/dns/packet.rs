use crate::utils::bytecursor::ByteCursor;

use super::{answer::DNSAnswer, header::DNSHeader, question::DNSQuestion};

/*
    Packet - encapsulates the header, query and answers into a coherent DNS packet.
    Exposes interfaces to directly read and write to byte streams.
*/
#[derive(Debug)]
pub struct DNSPacket {
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
    pub fn new(content: &[u8]) -> Result<Self, Box<dyn std::error::Error>>{
        let mut bytestream = ByteCursor::from_u8(content);
        let _header = DNSHeader::new(&mut bytestream).unwrap();
        // Get N questions from DNS header. 
        // println!("DNS Header: {:?}", _header);
        let mut _questions = Vec::new();
        for i in 0.._header._qdcount {
            let _question = DNSQuestion::new(&mut bytestream).unwrap();
            println!("DNS Query received =======");
            println!("{:?}", _question);
            _questions.push(_question);
        }
        
        // println!("DNS header read as  {:?} and DNS bytestream as {:?}", _header, bytestream);
        Ok(DNSPacket::default())
    }
}