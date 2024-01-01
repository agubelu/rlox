pub type OpCode = u8;
pub type Value = f64;

pub struct OpCodes;
impl OpCodes {
    pub const OP_RETURN: OpCode = 0;
    pub const OP_CONSTANT: OpCode = 1;
}