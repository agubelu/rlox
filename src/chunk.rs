use std::ops::Index;
use crate::common::Value;

pub struct Chunk {
    pub bytes: Vec<u8>,
    pub lines: Vec<usize>,
    pub values: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { bytes: vec![], lines: vec![], values: vec![] }
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.bytes.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl Index<usize> for Chunk {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}

impl Index<u8> for Chunk {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.bytes[index as usize]
    }
}