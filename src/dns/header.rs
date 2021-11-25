use std::{default, io::Cursor};

use crate::utils::bytecursor::ByteCursor;

// See implementation standard at: https://datatracker.ietf.org/doc/html/rfc5395
// Author: Sandesh Bhusal <mail.sandeshbhusal@gmail.com>
#[derive(Debug)]
pub enum OPCODE {
    QUERY,
    IQUERY,
    STATUS,
    NOTIFY,
    UPDATE,

    UNKNOWN,
}

#[derive(Debug)]
pub enum RETURNCODE {
    NOERR,
    FORMERR,
    SERVFAIL,
    NXDOMAIN,
    NOTIMP,
    REFUSED,
    YXDOMAIN,
    YRRSET,
    NXRRSET,
    NOTAUTH,
    NOTZONE,
    BADVERS,
    BADSIG,
    BADKEY,
    BADTIME,
    BADMODE,
    BADNAME,
    BADALG,
    BADTRUNC,

    UNKNOWN,
}

#[derive(Debug)]
pub struct DNSHeader {
    pub _id: u16,                   // 16 bit ID of the DNS Packet
    pub _qr: bool,                  // Is the packet query or answer
    pub _opcode: OPCODE,            // Opcode of the packet
    pub _authoritative: bool,       // Is the answer from an authoritative nameserver?
    pub _truncated: bool,           // Is the response truncated
    pub _recursion_desired: bool,   // Is recursion desired
    pub _recursion_available: bool, // Is recursion available in the nameserver
    pub _return_code: RETURNCODE,   // Return code from the DNS server

    pub _qdcount: u16, // Number of questions
    pub _ancount: u16, // Number of answers
    pub _nscount: u16, // Number of nameserver resource records
    pub _arcount: u16, // Number of additional records
}

impl Default for DNSHeader {
    // Returns a default empty DNS Header
    fn default() -> Self {
        return DNSHeader {
            _id: 0,
            _qr: false,
            _opcode: OPCODE::UNKNOWN,
            _authoritative: false,
            _truncated: false,
            _recursion_desired: false,
            _recursion_available: false,
            _return_code: RETURNCODE::UNKNOWN,
            _qdcount: 0,
            _ancount: 0,
            _nscount: 0,
            _arcount: 0,
        };
    }
}


impl DNSHeader {
    pub fn new(bytestream: &mut ByteCursor) -> Result<Self, Box<dyn std::error::Error>> {
        // extract id
        let _id = bytestream.read_u16().unwrap();
        let _qr = bytestream.peek_u8().unwrap() & 0b1000_0000 == 1;
        let _opcode = match bytestream.peek_u8().unwrap() & 0b0111_1000 {
            0 => OPCODE::QUERY,
            _ => OPCODE::UNKNOWN,
        };
        let _authoritative = bytestream.peek_u8().unwrap() & 0b0000_0100 == 1;
        let _truncated = bytestream.peek_u8().unwrap() & 0b0000_0010 == 1;
        let _recursion_desired = bytestream.read_u8().unwrap() & 0b0000_0001 == 1;
        let _recursion_available = bytestream.peek_u8().unwrap() & 0b1000_0000 == 1;
        let _return_code = match bytestream.read_u8().unwrap() & 0b0000_1111 {
            0 => RETURNCODE::NOERR,
            1 => RETURNCODE::FORMERR,
            2 => RETURNCODE::SERVFAIL,
            3 => RETURNCODE::NXDOMAIN,
            4 => RETURNCODE::NOTIMP,
            5 => RETURNCODE::REFUSED,

            _ => RETURNCODE::UNKNOWN,
        };

        let _qdcount = bytestream.read_u16().unwrap();
        let _ancount = bytestream.read_u16().unwrap();
        let _nscount = bytestream.read_u16().unwrap();
        let _arcount = bytestream.read_u16().unwrap();

        Ok(DNSHeader {
            _id,
            _qr,
            _opcode,
            _authoritative,
            _truncated,
            _recursion_desired,
            _recursion_available,
            _return_code,

            _qdcount,
            _ancount,
            _nscount,
            _arcount,
        })
    }
}
