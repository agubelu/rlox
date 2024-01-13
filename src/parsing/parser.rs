use crate::{Chunk, LoxValue, OpCodes};
use crate::scanning::{Scanner, Token, TokenType};
use super::utils::{Precedence, Precs};
use TokenType::*;

pub struct Parser<'src, 'chk> {
    scanner: Scanner<'src>,
    previous: Token<'src>,
    current: Token<'src>,
    chunk: Option<&'chk mut Chunk>,
    had_error: bool,
    panic_mode: bool,
}

impl<'src, 'chk> Parser<'src, 'chk> {
    pub fn new(source: &'src str) -> Self {
        Self {
            scanner: Scanner::new(source),
            previous: Token::default(),
            current: Token::default(),
            chunk: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(&mut self, chunk: &'chk mut Chunk) -> bool {
        self.chunk = Some(chunk);
        self.consume();
        self.expression();
        self.finish();
        !self.had_error
    }

    //////////////////////////////////////////////////////////////////////////////////////////
    /// Expression parsing methods

    fn parse_precedence(&mut self, prec: Precedence) {
        self.consume();

        // At this point, the parser is at the beginning of an expression.
        // Check the parsing rules to determine which method to use
        // to start parsing it according to the previous token.
        let (prefix_fn, _, _) = Self::get_rule(self.previous.kind);
        if prefix_fn.is_none() {
            self.error_at_previous("Expected expression.");
            return;
        }

        // Parse the infix expression
        prefix_fn.unwrap()(self);

        // At this point, the next token may be a binary operator. If it is and
        // it has a higher precedence than the expression we are currently
        // parsing, include it as well
        while Self::get_rule(self.current.kind).2 >= prec {
            // Consume the binary operator token
            self.consume();
            // Parse the expression
            let infix_fn = Self::get_rule(self.previous.kind).1.unwrap();
            infix_fn(self);
        }
    }

    pub(super) fn expression(&mut self) {
        self.parse_precedence(Precs::ASSIGN);
    }

    pub(super) fn literal(&mut self) {
        match self.previous.kind {
            Null => self.emit_byte(OpCodes::OP_NULL),
            True => self.emit_byte(OpCodes::OP_TRUE),
            False => self.emit_byte(OpCodes::OP_FALSE),
            _ => unreachable!()
        }
    }

    pub(super) fn number(&mut self) {
        // We can unwrap safely because the token wouldn't be of type Number
        // if the format wasn't correct.
        let number = self.previous.literal.parse().unwrap();
        self.emit_constant(LoxValue::Number(number));
    }

    pub(super) fn grouping(&mut self) {
        self.expression();
        self.consume_if(RightParen, "Expected closing ')' after expression.");
    }

    pub(super) fn unary(&mut self) {
        let op = self.previous.kind;

        // Compile the expression ahead first
        self.parse_precedence(Precs::UNARY);

        // Emit the right instruction according to the operand
        match op {
            Minus => self.emit_byte(OpCodes::OP_NEGATE),
            Not => self.emit_byte(OpCodes::OP_NOT),
            _ => unreachable!(),
        }
    }

    pub(super) fn binary(&mut self) {
        // The left-side expression has already been compiled
        let op = self.previous.kind;
        let (_, _, precedence) = Self::get_rule(op);

        // Compile the right-side expression with a higher precedence
        // to ensure left associativity
        self.parse_precedence(precedence + 1);

        // Emit the right instructions according to the operand
        match op {
            Plus => self.emit_byte(OpCodes::OP_ADD),
            Minus => self.emit_byte(OpCodes::OP_SUBSTRACT),
            Asterisk => self.emit_byte(OpCodes::OP_MULTIPLY),
            Slash => self.emit_byte(OpCodes::OP_DIVIDE),
            EqualEqual => self.emit_byte(OpCodes::OP_EQUAL),
            NotEqual => self.emit_bytes(OpCodes::OP_EQUAL, OpCodes::OP_NOT),
            Less => self.emit_byte(OpCodes::OP_LESS),
            LessEqual => self.emit_bytes(OpCodes::OP_GREATER, OpCodes::OP_NOT),
            Greater => self.emit_byte(OpCodes::OP_GREATER),
            GreaterEqual => self.emit_bytes(OpCodes::OP_LESS, OpCodes::OP_NOT),
            _ => unreachable!()
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////////
    /// Token processing methods

    /** Unconditionally consumes the next token from the scanner */
    fn consume(&mut self) {
        self.previous = self.current;
        self.current = self.scanner.scan_next_token();

        if self.current.kind == Error {
            self.error_at_current(self.current.literal);
        }
    }

    /** Consumes the next token from the scanner if it is of the expected type */
    fn consume_if(&mut self, expected: TokenType, msg: &str) {
        if self.current.kind == expected {
            self.consume();
        } else {
            self.error_at_current(msg);
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////////
    /// Bytecode compiling methods

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line;
        self.current_chunk().write_byte(byte, line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_constant(&mut self, val: LoxValue) {
        let ix = self.current_chunk().add_constant(val);
        if ix > u8::MAX as usize {
            panic!("Max constants reached.");
        }

        self.emit_bytes(OpCodes::OP_CONSTANT, ix as u8);
    }

    fn finish(&mut self) {
        self.emit_byte(OpCodes::OP_RETURN);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk.as_mut().unwrap()
    }

    //////////////////////////////////////////////////////////////////////////////////////////
    /// Error methods
    fn error_at_current(&mut self, msg: &str) {
        self.error_at(self.current, msg);
    }

    fn error_at_previous(&mut self, msg: &str) {
        self.error_at(self.previous, msg);
    }

    fn error_at(&mut self, token: Token, msg: &str) {
        if self.panic_mode { return }

        eprint!("[Line {}] Error", token.line);

        if token.kind == Eof {
            eprint!(" at end");
        } else if token.kind != Error {
            eprint!(" at '{}'", token.literal);
        }

        eprintln!(": {msg}");

        self.had_error = true;
        self.panic_mode = true;
    }

}

