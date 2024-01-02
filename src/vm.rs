use crate::chunk::Chunk;
use crate::common::{Value, OpCodes};

const STACK_SIZE: usize = 256;

pub struct VM {
    ip: usize,
    stack: [Value; 256],
    stack_top: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    // Macro for arithmetic operations in the VM that take two values.
    // It is generic over the arithmetic operator to use.
    ($self:ident, $op:tt) => {
        {
            let val = $self.pop() $op $self.pop();
            $self.push(val);
        }
    };
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: [Value::default(); STACK_SIZE],
            stack_top: 0
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.ip = 0;
        self.run(chunk)
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            #[cfg(feature = "trace")]
            self.trace(chunk);

            let op = self.read_byte(chunk);

            match op {
                OpCodes::OP_RETURN => {
                    let val = self.pop();
                    println!("{val}");
                    return InterpretResult::Ok
                },
                OpCodes::OP_CONSTANT => {
                    let val = self.read_constant(chunk);
                    self.push(val);
                },
                OpCodes::OP_NEGATE => {
                    let val = -self.pop();
                    self.push(val);
                },
                OpCodes::OP_ADD => binary_op!(self, +),
                OpCodes::OP_SUBSTRACT => binary_op!(self, -),
                OpCodes::OP_MULTIPLY => binary_op!(self, *),
                OpCodes::OP_DIVIDE => binary_op!(self, /),
                _ => {},
            }
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let ix = self.read_byte(chunk);
        chunk.values[ix as usize]
    }

    fn push(&mut self, val: Value) {
        self.stack[self.stack_top] = val;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    /// Conditional debug features

    #[cfg(feature = "trace")]
    fn trace(&self, chunk: &Chunk) {
        self.trace_stack();
        self.trace_instr(chunk);
    }

    #[cfg(feature = "trace")]
    fn trace_stack(&self) {
        print!("[ ");
        for v in &self.stack[..self.stack_top] {
            print!("{v} ");
        }
        println!("] ");
    }

    #[cfg(feature = "trace")]
    fn trace_instr(&self, chunk: &Chunk) {
        use std::io::stdout;
        use crate::debug::debug_instruction;

        let mut f = stdout().lock();
        debug_instruction(&mut f, chunk, self.ip);
    }
}
