//! Packet library for DNS Server.

use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
struct DNSPacket {
    header: DNSHeader,
}

#[derive(Debug, DekuRead, DekuWrite, Clone, Copy)]
pub struct DNSHeader {
    #[deku(bits = "16")]
    id: u16,
    #[deku(bits = "1")]
    is_response: bool,
    #[deku(bits = "4")]
    opcode: u8,
    #[deku(bits = "1")]
    authoritative: bool,
    #[deku(bits = "1")]
    truncation: bool,
    #[deku(bits = "1")]
    recursion_desired: bool,
    #[deku(bits = "1")]
    recursion_available: bool,
    #[deku(bits = "3")]
    reserved: u8,
    #[deku(bits = "4")]
    response_code: u8,

    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

#[derive(Debug)]
pub enum OpCode {
    Query,
    IQuery,
    Status,
    Notify,
    Update,
    DSO,
    Unassigned,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::Query,
            1 => Self::IQuery,
            2 => Self::Status,
            4 => Self::Notify,
            5 => Self::Update,
            6 => Self::DSO,

            _ => Self::Unassigned,
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            Self::Query => 0,
            Self::IQuery => 1,
            Self::Status => 2,
            Self::Notify => 3,
            Self::Update => 4,
            Self::DSO => 5,

            Self::Unassigned => 6,
        }
    }
}

pub enum RCode {
    NOERR,
    FORMERR,
    SERVFAIL,
    NXDOMAIN,
    NOTIMP,
    REFUSED,
    YXDOMAIN,
    YXRRSET,
    NXRRSET,
    NOTAUTH,
    NOTZONE,
    DSOTYPEINI,
    BADVERS,
    BADSIG,
    BADTIME,
    BADMODE,
    BADNAME,
    BADALG,
    BADTRUNC,
    BADCOOKIE,
    UNASSIGNED,
}

// Partial implementation; Complete later.
impl From<u8> for RCode {
    fn from(rcode: u8) -> Self {
        match rcode {
            0 => Self::NOERR,
            1 => Self::FORMERR,
            2 => Self::SERVFAIL,
            3 => Self::NXDOMAIN,
            4 => Self::NOTIMP,

            _ => Self::UNASSIGNED,
        }
    }
}

// Partial implementation; complete later.
impl Into<u8> for RCode {
    fn into(self) -> u8 {
        match self {
            Self::NOERR => 0,
            Self::FORMERR => 1,
            Self::SERVFAIL => 2,
            Self::NXDOMAIN => 3,
            Self::NOTIMP => 4,

            _ => 12,
        }
    }
}

struct DNSAnswer;
struct DNSQuestion;

impl DNSPacket {
    fn new() -> Self {
        todo!()
    }

    fn answers() -> Vec<DNSAnswer> {
        todo!()
    }

    fn questions() -> Vec<DNSQuestion> {
        todo!()
    }

    fn header(&self) -> DNSHeader {
        self.header
    }
}

#[cfg(test)]
mod tests {
    use deku::DekuContainerWrite;

    use super::DNSPacket;

    #[test]
    fn print_dns_packet() {
        let packet = DNSPacket {
            header: super::DNSHeader {
                id: 0x100,
                is_response: false,
                opcode: 10,
                authoritative: true,
                truncation: true,
                recursion_desired: true,
                recursion_available: false,
                reserved: 0x0,
                response_code: 112,
                qdcount: 0,
                ancount: 0,
                nscount: 1,
                arcount: 2,
            },
        };

        dbg!(packet.header.to_bytes().unwrap());
    }
}
