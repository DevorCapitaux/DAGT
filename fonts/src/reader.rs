use std::{fs::File, io::Read};

pub struct Reader {
    buffer: Vec<u8>,
    cursor: usize,
}

impl Reader {
    pub fn new(file: &mut File) -> Reader {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Reader { buffer, cursor: 0 }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn skip(&mut self, val: usize) {
        self.cursor += val;
    }
    //
    pub fn goto(&mut self, val: usize) {
        self.cursor = val;
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.buffer[self.cursor];
        self.cursor += 1;
        val
    }

    pub fn read_u16(&mut self) -> u16 {
        let val = u16::from_be_bytes([self.buffer[self.cursor], self.buffer[self.cursor + 1]]);
        self.cursor += 2;
        val
    }

    pub fn read_i16(&mut self) -> i16 {
        let val = i16::from_be_bytes([self.buffer[self.cursor], self.buffer[self.cursor + 1]]);
        self.cursor += 2;
        val
    }

    pub fn read_u32(&mut self) -> u32 {
        let val = u32::from_be_bytes([
            self.buffer[self.cursor],
            self.buffer[self.cursor + 1],
            self.buffer[self.cursor + 2],
            self.buffer[self.cursor + 3],
        ]);
        self.cursor += 4;
        val
    }

    pub fn read_string(&mut self, bytes: usize) -> String {
        let string = std::str::from_utf8(&self.buffer[self.cursor..self.cursor + bytes]).unwrap();
        self.cursor += bytes;
        string.to_owned()
    }
}
