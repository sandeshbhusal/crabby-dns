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
pub struct DNSQuestion {
    _name: String,
    _type: TYPE,
    _class: CLASS
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

        Ok(DNSQuestion{
            _name,
            _type,
            _class
        })
    }
}