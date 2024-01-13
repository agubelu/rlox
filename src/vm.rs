use crate::parsing::Parser;
use crate::{Chunk, OpCodes, LoxValue};

const STACK_SIZE: usize = 256;

pub struct VM {
    ip: usize,
    stack: [LoxValue; 256],
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
            if let Some(lox_val) = $self.pop() $op $self.pop() {
                $self.push(lox_val);
            } else {
                runtime_error!($self, "Values must be numbers.");
            }
        }
    };
}

macro_rules! runtime_error {
    ($self:ident, $msg:tt) => {
        {
            $self.runtime_error($msg);
            return InterpretResult::RuntimeError;
        }
    };
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: [LoxValue::Null; STACK_SIZE],
            stack_top: 0
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut parser = Parser::new(source);
        let mut chunk = Chunk::new();
        self.ip = 0;

        if !parser.compile(&mut chunk) {
            return InterpretResult::CompileError;
        }

        self.run(&chunk)
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
                    if let Some(lox_val) = -self.pop() {
                        self.push(lox_val);
                    } else {
                        runtime_error!(self, "Value must be a number.");
                    }
                },
                OpCodes::OP_ADD => binary_op!(self, +),
                OpCodes::OP_SUBSTRACT => binary_op!(self, -),
                OpCodes::OP_MULTIPLY => binary_op!(self, *),
                OpCodes::OP_DIVIDE => binary_op!(self, /),
                OpCodes::OP_NULL => self.push(LoxValue::Null),
                OpCodes::OP_TRUE => self.push(LoxValue::Bool(true)),
                OpCodes::OP_FALSE => self.push(LoxValue::Bool(false)),
                OpCodes::OP_NOT => {
                    let val = self.pop().is_falsey();
                    self.push(LoxValue::Bool(val));
                },
                OpCodes::OP_EQUAL => {
                    let val = self.pop() == self.pop();
                    self.push(LoxValue::Bool(val));
                },
                OpCodes::OP_GREATER => {
                    if let Some(lox_val) = self.pop().greater(&self.pop()) {
                        self.push(lox_val);
                    } else {
                        runtime_error!(self, "Values must be numbers.");
                    }
                },
                OpCodes::OP_LESS => {
                    if let Some(lox_val) = self.pop().less(&self.pop()) {
                        self.push(lox_val);
                    } else {
                        runtime_error!(self, "Values must be numbers.");
                    }
                },
                _ => {},
            }
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self, chunk: &Chunk) -> LoxValue {
        let ix = self.read_byte(chunk);
        chunk.values[ix as usize]
    }

    fn push(&mut self, val: LoxValue) {
        self.stack[self.stack_top] = val;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> LoxValue {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn peek(&self, dist: usize) -> LoxValue {
        self.stack[self.stack_top - 1 - dist]
    }

    fn runtime_error(&self, msg: &str) {
        eprintln!("{msg}");
        // TODO
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

impl InterpretResult {
    pub fn exit_code(&self) -> i32 {
        match self {
            InterpretResult::Ok => 0,
            InterpretResult::CompileError => 65,
            InterpretResult::RuntimeError => 70,
        }
    }
}