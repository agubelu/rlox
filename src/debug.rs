use std::fmt::{Debug, Formatter, Result};
use crate::common::OpCodes;
use crate::chunk::Chunk;

type Fmtr<'a, 'b> = &'a mut Formatter<'b>;

impl Debug for Chunk {
    fn fmt(&self, f: Fmtr) -> Result {
        let mut offset = 0;
        while offset < self.len() {
            offset = debug_instruction(f, self, offset);
        }
        Ok(())
    }
}

fn debug_instruction(f: Fmtr, chunk: &Chunk, offset: usize) -> usize {
    write!(f, "{offset:04} ").unwrap();

    // Print out the line number, or a vertical bar if it's the same
    // line as the previous instruction
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        write!(f, "   | ").unwrap();
    } else {
        let line = chunk.lines[offset];
        write!(f, "{line:>4} ").unwrap();
    }

    let opcode = chunk[offset];
    match opcode {
        OpCodes::OP_RETURN => simple_op(f, "OP_RETURN", offset),
        OpCodes::OP_CONSTANT => constant_op(f, "OP_CONSTANT", chunk, offset),
        _ => panic!("Unknown opcode: {opcode}"),
    }
}

fn simple_op(f: Fmtr, name: &str, offset: usize) -> usize {
    writeln!(f, "{name}").unwrap();
    offset + 1
}

fn constant_op(f: Fmtr, name: &str, chunk: &Chunk, offset: usize) -> usize {
    let value_ix = chunk[offset + 1];
    let value = chunk.values[value_ix as usize];
    writeln!(f, "{name:<16} {value_ix:4} '{value}'").unwrap();
    offset + 2
}