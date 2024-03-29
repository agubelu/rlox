use std::mem::take;
use crate::parsing::Parser;
use crate::runtime::{Chunk, OpCodes};
use crate::values::LoxValue;

const STACK_SIZE: usize = 256;

pub struct VM {
    ip: usize,
    stack: [LoxValue; STACK_SIZE],
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
    ($self:ident, $chunk:ident, $op:tt) => {
        {
            if let Some(lox_val) = $self.pop() $op $self.pop() {
                $self.push(lox_val);
            } else {
                runtime_error!($self, $chunk, "Values have incompatible types.");
            }
        }
    };
}

macro_rules! runtime_error {
    ($self:ident, $chunk:ident, $($args:expr),+) => {
        {
            eprintln!($($args),+);
            let line_err = $chunk.lines[$self.ip - 1];
            eprintln!("[line {line_err}] in script.");
            $self.reset_stack();
            return InterpretResult::RuntimeError;
        }
    };
}

impl VM {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: std::array::from_fn(|_| LoxValue::default()),
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
                        runtime_error!(self, chunk, "Value must be a number.");
                    }
                },
                OpCodes::OP_ADD => binary_op!(self, chunk, +),
                OpCodes::OP_SUBSTRACT => binary_op!(self, chunk, -),
                OpCodes::OP_MULTIPLY => binary_op!(self, chunk, *),
                OpCodes::OP_DIVIDE => binary_op!(self, chunk, /),
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
                        runtime_error!(self, chunk, "Values must be numbers.");
                    }
                },
                OpCodes::OP_LESS => {
                    if let Some(lox_val) = self.pop().less(&self.pop()) {
                        self.push(lox_val);
                    } else {
                        runtime_error!(self, chunk, "Values must be numbers.");
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
        chunk.values[ix as usize].clone()
    }

    fn push(&mut self, val: LoxValue) {
        self.stack[self.stack_top] = val;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> LoxValue {
        // TODO: make it Copy again?
        self.stack_top -= 1;
        // When popping a value, we replace it with Null on the stack
        // as it won't be used anymore. This ensures that values with
        // allocated memory on the heap are moved out of the stack and
        // their data is dropped when the value itself is after being used.
        take(&mut self.stack[self.stack_top])
    }

    fn reset_stack(&mut self) {
        self.stack_top = 0;
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