/*
    Implementation of a byte cursor object.
    Takes in &[u8] and returns a byte cursor. It supports the given operations:
    1. Get next u8 (peek or get)
    2. Get next u16
    3. Get next u32
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
    pub fn from_u8(bytestream: &[u8]) -> Self{
        let _len = bytestream.len();
        ByteCursor {
            position: 0,
            length: _len,
            content: Vec::from(bytestream),
        }
    }
    
    pub fn new() -> ByteCursor {
        ByteCursor::default()
    }

    pub fn peek_u8(&mut self) -> Result<u8, ByteReadError> { 
        // peek_u8 returns the u8 value at current "position". Position guaranteed to be within bounds. 
        if self.length == 0 {
            return Err(ByteReadError{});
        }
        Ok(self.content[self.position])
    }

    pub fn read_u8(&mut self) -> Result<u8, ByteReadError> { 
        let _byte = self.peek_u8()?;
        if self.position == self.length - 1 {
            // Out of bytes. 
            return Err(ByteReadError{});
        }
        self.position += 1;
        Ok(_byte)
    }

    pub fn read_u16(&mut self) -> Result<u16, ByteReadError> { 
        let _byte1 = self.read_u8()?;
        let _byte2 = self.read_u8()?;

        Ok((_byte1 as u16) << 8 | _byte2 as u16)
    }

    pub fn read_u32(&mut self) -> Result<u32, ByteReadError> { 
        let _double1 = self.read_u16()?;
        let _double2 = self.read_u16()?;

        Ok((_double1 as u32) << 16 | _double2 as u32)
    }

    pub fn goto_offset(&mut self, offset: usize) -> Result<(), ByteReadError> {
        if offset < 0 || offset > self.length - 1 {
            // raise an error
            return Err(ByteReadError{});
        }
        self.position = offset;
        Ok(())
    }

    pub fn rewind(&mut self) {
        self.position = 0;
    }
}

impl From<&[u8]> for ByteCursor{
    fn from(bytestream: &[u8]) -> Self {
        return ByteCursor::from_u8(bytestream);
    }
}