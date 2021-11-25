#[derive(Debug)]
enum TYPE {
    A,
    AAAA,
    MX,
    CNAME,
    NS,

    UNKNOWN
}

#[derive(Debug)]
enum CLASS {
    RESERVED,
    IN,
    CH,
    HS,
    NONE,

    UNKNOWN
}

#[derive(Debug)]
enum RDATA {
    IP([u8; 4]),
    CNAME(String),

    UNKNOWN(String)
}

#[derive(Debug)]
pub struct DNSAnswer {
    _name: String,
    _type: TYPE,
    _class: CLASS,
    _ttl : u32,
    _rdlength: u16,
    _rdata: RDATA
}

impl Default for DNSAnswer {
    fn default() -> Self { 
        return DNSAnswer {
            _name: String::from(""),
            _type: TYPE::UNKNOWN,
            _class: CLASS::UNKNOWN,
            _ttl: 0,
            _rdlength:0,
            _rdata: RDATA::UNKNOWN(String::from("default answer"))
        }
    }
}

impl DNSAnswer{
    pub fn new(bytestream: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut _name = String::new();
        let mut _i = 0usize; // index for the bytestream

        while bytestream[_i] != 0 {
            // Read number of bytes from _i. 
            let mut _readable = bytestream[_i];
            _i += 1;
            while _readable > 0 {
                _name.push(bytestream[_i] as char);
                _i += 1;
                _readable -= 1;
            }

            _name.push('.');
        }
        
        _name = String::from(_name.strip_suffix('.').unwrap());

        _i += 1; // Skip the null termination character. 
        
        let _type = match (bytestream[_i] as u16) << 8 | bytestream[_i + 1] as u16 {
            1 => TYPE::A,
            5 => TYPE::NS,
            _ => TYPE::UNKNOWN
        };
        
        _i += 2;

        let _class = match (bytestream[_i] as u16) << 8 | bytestream[_i + 1] as u16 {
            1 => CLASS::IN,
            _ => CLASS::UNKNOWN
        };

        _i += 2;
        
        let _ttl = (bytestream[_i] as u32) << 24 | (bytestream[_i+1] as u32) << 16 | (bytestream[_i+2] as u32) << 8 |   (bytestream[_i+3] as u32);

        _i += 4;

        let _rdlength = (bytestream[_i] as u16) << 8 | bytestream[_i+1] as u16;
        let _rdata = match _type {
            TYPE::A => {
                RDATA::IP([1, 2, 3, 4])
            }
            _ => {
                RDATA::UNKNOWN(String::from(format!("Unkown _type {:?} of dns packet", _type)))
            }
        };

        Ok(DNSAnswer{
            _name,
            _type,
            _class,
            _ttl,
            _rdlength,
            _rdata
        })
    }
}