pub type OpCode = u8;
pub type Value = f64;

pub struct OpCodes;
impl OpCodes {
    pub const OP_RETURN: OpCode = 0;
    pub const OP_CONSTANT: OpCode = 1;
    pub const OP_NEGATE: OpCode = 2;
    pub const OP_ADD: OpCode = 3;
    pub const OP_SUBSTRACT: OpCode = 4;
    pub const OP_MULTIPLY: OpCode = 5;
    pub const OP_DIVIDE: OpCode = 6;
}

pub enum LoxValue {
    Bool(bool),
    Number(f64),
    Null,
}