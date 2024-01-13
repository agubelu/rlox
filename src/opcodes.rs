pub type OpCode = u8;

pub struct OpCodes;
impl OpCodes {
    pub const OP_RETURN: OpCode = 0;
    pub const OP_CONSTANT: OpCode = 1;
    pub const OP_NEGATE: OpCode = 2;
    pub const OP_ADD: OpCode = 3;
    pub const OP_SUBSTRACT: OpCode = 4;
    pub const OP_MULTIPLY: OpCode = 5;
    pub const OP_DIVIDE: OpCode = 6;
    pub const OP_NULL: OpCode = 7;
    pub const OP_TRUE: OpCode = 8;
    pub const OP_FALSE: OpCode = 9;
    pub const OP_NOT: OpCode = 10;
    pub const OP_EQUAL: OpCode = 11;
    pub const OP_GREATER: OpCode = 12;
    pub const OP_LESS: OpCode = 13;
}