mod chunk;
mod common;
mod debug;
mod vm;

use chunk::Chunk;
use common::OpCodes;
use vm::VM;

fn main() {
    let mut chonky = Chunk::new();

    let ix1 = chonky.add_constant(1.0);
    let ix2 = chonky.add_constant(2.0);
    let ix3 = chonky.add_constant(3.0);

    chonky.write_byte(OpCodes::OP_CONSTANT, 0);
    chonky.write_byte(ix1 as u8, 0);

    chonky.write_byte(OpCodes::OP_NEGATE, 0);

    chonky.write_byte(OpCodes::OP_CONSTANT, 0);
    chonky.write_byte(ix2 as u8, 0);

    chonky.write_byte(OpCodes::OP_MULTIPLY, 0);

    chonky.write_byte(OpCodes::OP_CONSTANT, 0);
    chonky.write_byte(ix3 as u8, 0);

    chonky.write_byte(OpCodes::OP_ADD, 0);
    chonky.write_byte(OpCodes::OP_RETURN, 0);

    let mut vm = VM::new();
    println!("{:?}", vm.interpret(&chonky));
}
