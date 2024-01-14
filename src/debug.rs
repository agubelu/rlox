// Implementations of Debug and Display that are useful to
// know what's going on inside the VM as it runs.

use std::fmt::{Debug, Formatter, Result, Display};
use std::io::Write;
use crate::runtime::{OpCodes, Chunk};
use crate::values::{LoxValue, LoxObject};

type Fmtr<'a, 'b> = &'a mut Formatter<'b>;

impl Debug for Chunk {
    fn fmt(&self, f: Fmtr) -> Result {
        let mut offset = 0;
        let mut writer = IOFormatter::new(f);

        while offset < self.len() {
            offset = debug_instruction(&mut writer, self, offset);
        }
        Ok(())
    }
}

pub fn debug_instruction(f: &mut impl Write, chunk: &Chunk, offset: usize) -> usize {
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
        OpCodes::OP_NEGATE => simple_op(f, "OP_NEGATE", offset),
        OpCodes::OP_ADD => simple_op(f, "OP_ADD", offset),
        OpCodes::OP_SUBSTRACT => simple_op(f, "OP_SUBSTRACT", offset),
        OpCodes::OP_MULTIPLY => simple_op(f, "OP_MULTIPLY", offset),
        OpCodes::OP_DIVIDE => simple_op(f, "OP_DIVIDE", offset),
        OpCodes::OP_NULL => simple_op(f, "OP_NULL", offset),
        OpCodes::OP_TRUE => simple_op(f, "OP_TRUE", offset),
        OpCodes::OP_FALSE => simple_op(f, "OP_FALSE", offset),
        OpCodes::OP_NOT => simple_op(f, "OP_NOT", offset),
        OpCodes::OP_EQUAL => simple_op(f, "OP_EQUAL", offset),
        OpCodes::OP_LESS => simple_op(f, "OP_LESS", offset),
        OpCodes::OP_GREATER => simple_op(f, "OP_GREATER", offset),
        _ => panic!("Unknown opcode: {opcode}"),
    }
}

fn simple_op(f: &mut impl Write, name: &str, offset: usize) -> usize {
    writeln!(f, "{name}").unwrap();
    offset + 1
}

fn constant_op(f: &mut impl Write, name: &str, chunk: &Chunk, offset: usize) -> usize {
    let value_ix = chunk[offset + 1];
    let value = &chunk.values[value_ix as usize];
    writeln!(f, "{name:<16} {value_ix:4} '{value}'").unwrap();
    offset + 2
}

////////////////////////////////////////////////////////////////////////////

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Object(o) => write!(f, "{o}"),
            Self::Null => write!(f, "null"),
        }
    }
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
}

impl Debug for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

////////////////////////////////////////////////////////////////////////////
// Aux struct for implementing io::Write in std::Formatter

struct IOFormatter<'a, 'b> {
    f: &'a mut Formatter<'b>
}

impl<'a, 'b> IOFormatter<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>) -> Self {
        Self { f }
    }
}

impl<'a, 'b> Write for IOFormatter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.f.write_str(&String::from_utf8_lossy(buf))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
