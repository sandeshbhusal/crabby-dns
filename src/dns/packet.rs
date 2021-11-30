use crate::utils::bytecursor::ByteCursor;

use super::{header::DNSHeader, question::DNSQuestion};

#[derive(Debug)]
pub struct DNSPacket {
    _header: DNSHeader,
    _questions: Vec<DNSQuestion>
}

impl Default for DNSPacket{
    fn default() -> Self { 
        DNSPacket{
            _header: DNSHeader::default(),
            _questions: vec![DNSQuestion::default()],
        }
    }
}

impl DNSPacket{
    // Create a DNS packet from a slice of u8
    pub fn new(content: &[u8]) -> Result<Self, Box<dyn std::error::Error>>{
        let mut cursor = ByteCursor::from_u8(content);
        let _header = DNSHeader::new(&mut cursor).unwrap();
        
        // Get N questions from DNS header. 
        // println!("DNS Header: {:?}", _header);
        let mut _questions = Vec::new();
        
        for i in 0.._header._qdcount {
            let _question = DNSQuestion::new(&mut cursor).unwrap();
            println!("DNS Query received =======");
            println!("{:?}", _question);
            let _bytes = _question.to_bytes();
            println!("{:?} was header bytes \n{:?} was question encoded", &content[12..], _bytes);
            _questions.push(_question);
        }
        
        // println!("DNS header read as  {:?} and DNS bytestream as {:?}", _header, bytestream);
        Ok(DNSPacket::default())
    }
}