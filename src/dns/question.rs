use crate::utils::bytecursor::ByteCursor;

#[derive(Debug, Clone, Copy)]
pub enum TYPE {
    A = 0,
    AAAA,
    MX,
    CNAME,
    NS,

    UNKNOWN
}

#[derive(Debug, Clone, Copy)]
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
    pub fn new(cursor: &mut ByteCursor) -> Result<Self, Box<dyn std::error::Error>> {
        let mut _name = String::new();

        while cursor.peek_u8().unwrap() != 0 {
            // Read number of bytes from _i. 
            let mut count = cursor.read_u8().unwrap();
            while count > 0 {
                _name.push(cursor.read_u8().unwrap() as char);
                count -= 1;
            }
            
            _name.push('.');
        }
        
        cursor.read_u8().unwrap(); // Pop the last zero off. (Null termination.)
        
        let _type = match cursor.read_u16().unwrap() {
            1 => TYPE::A,
            5 => TYPE::NS,
            _ => TYPE::UNKNOWN
        };
        
        let _class = match cursor.read_u16().unwrap() {
            1 => CLASS::IN,
            _ => CLASS::UNKNOWN
        };

        Ok(DNSQuestion{
            _name,
            _type,
            _class
        })
    }

    pub fn to_bytes (&self) -> Vec<u8> {
        // This function converts the given DNS question struct into a vector of bytes. 
        let mut _packet = Vec::new();

        // Encode name
        self._name.split('.').into_iter().for_each(|z| {
            let _l = z.len();
            println!("Pushing '{}' {}", z, &_l);
            _packet.push(_l as u8);
            z[..].as_bytes().iter().for_each(|b| _packet.push(*b));
        });

        // Encode type
        _packet.push(self._type as u8);
        
        // Encode class
        _packet.push(self._class as u8);

        // Return final results.
        _packet
    }
}