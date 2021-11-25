use crate::utils::bytecursor::ByteCursor;

#[derive(Debug)]
pub enum TYPE {
    A,
    AAAA,
    MX,
    CNAME,
    NS,

    UNKNOWN
}

#[derive(Debug)]
pub enum CLASS {
    RESERVED,
    IN,
    CH,
    HS,
    NONE,

    UNKNOWN
}

#[derive(Debug)]
pub struct DNSQuestion {
    pub _name: String,
    pub _type: TYPE,
    pub _class: CLASS
}

impl Default for DNSQuestion {
    fn default() -> Self { 
        return DNSQuestion {
            _name: String::from(""),
            _type: TYPE::UNKNOWN,
            _class: CLASS::UNKNOWN
        }
    }
}

impl DNSQuestion{
    pub fn new(bytestream: &mut ByteCursor) -> Result<Self, Box<dyn std::error::Error>> {
        let mut _name = String::new();

        while bytestream.peek_u8().unwrap() != 0 {
            // Read number of bytes from _i. 
            let mut _readable = bytestream.read_u8().unwrap();
            while _readable > 0 {
                _name.push(bytestream.read_u8().unwrap() as char);
                _readable -= 1;
            }

            _name.push('.');
        }
        
        bytestream.read_u8().unwrap(); // Pop the last character off. (Null termination.)

        // _name = String::from(_name.strip_suffix('.').unwrap());
        
        let _type = match bytestream.read_u16().unwrap() {
            1 => TYPE::A,
            5 => TYPE::NS,
            _ => TYPE::UNKNOWN
        };
        
        let _class = match bytestream.read_u16().unwrap() {
            1 => CLASS::IN,
            _ => CLASS::UNKNOWN
        };

        Ok(DNSQuestion{
            _name,
            _type,
            _class
        })
    }
}