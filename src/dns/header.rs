use std::{default, io::Cursor};

// See implementation standard at: https://datatracker.ietf.org/doc/html/rfc5395
// Author: Sandesh Bhusal <mail.sandeshbhusal@gmail.com>
#[derive(Debug)]
enum OPCODE {
    QUERY,
    IQUERY,
    STATUS,
    NOTIFY,
    UPDATE,

    UNKNOWN,
}

#[derive(Debug)]
enum RETURNCODE {
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
    _id: u16,                   // 16 bit ID of the DNS Packet
    _qr: bool,                  // Is the packet query or answer
    _opcode: OPCODE,            // Opcode of the packet
    _authoritative: bool,       // Is the answer from an authoritative nameserver?
    _truncated: bool,           // Is the response truncated
    _recursion_desired: bool,   // Is recursion desired
    _recursion_available: bool, // Is recursion available in the nameserver
    _return_code: RETURNCODE,   // Return code from the DNS server

    _qdcount: u16, // Number of questions
    _ancount: u16, // Number of answers
    _nscount: u16, // Number of nameserver resource records
    _arcount: u16, // Number of additional records
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
    pub fn new(bytestream: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let _vec = Vec::from(bytestream);
        // extract id
        let _id = (_vec[0] as u16) << 8 | (_vec[1] as u16);
        let _qr = _vec[2] & 0b1000_0000 == 1;
        let _opcode = match _vec[2] & 0b0111_1000 {
            0 => OPCODE::QUERY,
            _ => OPCODE::UNKNOWN,
        };
        let _authoritative = _vec[2] & 0b0000_0100 == 1;
        let _truncated = _vec[2] & 0b0000_0010 == 1;
        let _recursion_desired = _vec[2] & 0b0000_0001 == 1;
        let _recursion_available = _vec[3] & 0b1000_0000 == 1;
        let _return_code = match _vec[3] & 0b0000_1111 {
            0 => RETURNCODE::NOERR,
            1 => RETURNCODE::FORMERR,
            2 => RETURNCODE::SERVFAIL,
            3 => RETURNCODE::NXDOMAIN,
            4 => RETURNCODE::NOTIMP,
            5 => RETURNCODE::REFUSED,

            _ => RETURNCODE::UNKNOWN,
        };

        let _qdcount = (_vec[4] as u16) << 8 | _vec[5] as u16;
        let _ancount = (_vec[6] as u16) << 8 | _vec[7] as u16;
        let _nscount = (_vec[8] as u16) << 8 | _vec[9] as u16;
        let _arcount = (_vec[10] as u16) << 8 | _vec[11] as u16;

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
