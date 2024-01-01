mod chunk;
mod common;
mod debug;

use chunk::Chunk;
use common::OpCodes;

fn main() {
    let mut chonky = Chunk::new();

    let cix = chonky.add_constant(420.0);
    chonky.write_byte(OpCodes::OP_CONSTANT, 123);
    chonky.write_byte(cix as u8, 123);
    chonky.write_byte(OpCodes::OP_RETURN, 123);

    println!("{chonky:?}");
}
