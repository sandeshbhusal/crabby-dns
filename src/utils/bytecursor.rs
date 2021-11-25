/*
    Implementation of a byte cursor object.
    Takes in &[u8] and returns a byte cursor. It supports the given operations:
    1. Get next u8
    2. Get next u16
    3. Get next u32
    4. Get next n-bits as u8 (position does not change)
    5. Get next n-u8s as &[u8] or Vec<u8>
*/

#[derive(Debug)]
pub struct ByteCursor {
    position: usize,
    length: usize,
    content: Vec<u8>,
}

impl Default for ByteCursor {
    fn default() -> Self {
        ByteCursor {
            position: 0_usize,
            length: 0_usize,
            content: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByteReadError{}

impl std::fmt::Display for ByteReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No more bytes to read")
    }
}

impl ByteCursor {
    fn from_u8(bytestream: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let _len = bytestream.len();
        Ok(ByteCursor {
            position: 0,
            length: _len,
            content: Vec::from(bytestream),
        })
    }
    
    fn new() -> ByteCursor {
        ByteCursor::default()
    }

    fn read_u8(&mut self) -> Result<u8, ByteReadError> { 
        if self.position - 1 == self.length {
            // raise an error
            return Err(ByteReadError{});
        }
        let _ret = self.content[self.position];
        self.position += 1;
        return Ok(_ret);
    }

    fn read_u16(&mut self) -> Result<u16, ByteReadError> { 
        if self.position - 2 == self.length {
            return Err(ByteReadError{});
        }
        let _u1 = self.content[self.position];
        let _u2 = self.content[self.position + 1];
        self.position += 2;
        Ok((_u1 as u16) << 8 | (_u2 as u16))
    }

    fn read_u32(&mut self) -> Result<u32, ByteReadError> { 
        if self.position - 4 == self.length {
            return Err(ByteReadError{});
        }
        let _u1 = self.content[self.position];
        let _u2 = self.content[self.position + 1];
        let _u3 = self.content[self.position + 2];
        let _u4 = self.content[self.position + 3];
        self.position += 4;
        Ok((_u1 as u32) << 24 | (_u2 as u32) << 16 | (_u3 as u32) << 8 | _u4 as u32)
    }
}

#[test]
fn sometest() {
    assert!(1 == 1);
}