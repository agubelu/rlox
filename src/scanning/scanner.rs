use super::{Token, TokenType::{self, *}};

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u32,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, start: 0, current: 0, line: 1 }
    }

    pub fn scan_next_token(&mut self) -> Token<'src> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(Eof);
        }

        match self.consume() {
            "{" => self.make_token(LeftBrace),
            "}" => self.make_token(RightBrace),
            "(" => self.make_token(LeftParen),
            ")" => self.make_token(RightParen),
            "," => self.make_token(Comma),
            "." => self.make_token(Dot),
            "-" => self.make_token(Minus),
            "+" => self.make_token(Plus),
            ";" => self.make_token(Semicolon),
            "*" => self.make_token(Asterisk),
            "/" => self.make_token(Slash), // Comments are handled by skip_whitespace()
            "!" => self.make_token_if("=", NotEqual, Not),
            "=" => self.make_token_if("=", EqualEqual, Equal),
            ">" => self.make_token_if("=", GreaterEqual, Greater),
            "<" => self.make_token_if("=", LessEqual, Less),
            "\"" | "'" => self.make_string(),
            s if is_digit(s) => self.make_number(),
            s if is_alpha(s) => self.make_identifier(),
             _ => self.make_error_token("Unexpected character."),
        }
   }

    /////////////////////////////////////////////////////////////////////////////////////
    // Aux functions

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                " " | "\t" | "\r" => {
                    self.consume();
                },
                "\n" => {
                    self.consume();
                    self.line += 1;
                },
                "/" if self.peek_forward() == "/" => {
                    // Line comment, skip forward til the end of the line
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.consume();
                    }
                }
                _ => return,
            }
        }
    }

    fn make_token(&self, kind: TokenType) -> Token<'src> {
        let literal = &self.source[self.start .. self.current];
        Token { literal, kind, line: self.line }
    }

    fn make_token_if(&mut self, expected: &str, yes: TokenType, no: TokenType) -> Token<'src> {
        let kind = if self.matches(expected) { yes } else { no };
        self.make_token(kind)
    }

    fn make_string(&mut self) -> Token<'src> {
        let closing = char(self.peek_back());
        let mut peek;

        while !self.is_at_end() && {peek = self.peek(); char(peek)} != closing {
            if peek == "\n" {
                self.line += 1;
            }

            self.consume();
        }

        if self.is_at_end() {
            self.make_error_token("Unterminated string at the end of file.")
        } else {
            // Consume the closing quote
            self.consume();
            self.make_token(String)
        }
    }

    fn make_number(&mut self) -> Token<'src> {
        while is_digit(self.peek()) {
            self.consume();
        }

        if self.peek() == "." && is_digit(self.peek_forward()) {
            // Consume the decimals dot
            self.consume();

            while is_digit(self.peek()) {
                self.consume();
            }
        }

        self.make_token(Number)
    }

    fn make_identifier(&mut self) -> Token<'src> {
        let mut peek;
        while {peek = self.peek(); is_digit(peek) || is_alpha(peek)} {
            self.consume();
        }

        let literal = &self.source[self.start .. self.current];
        self.make_token(TokenType::from_literal(literal))
    }

    fn make_error_token(&self, literal: &'src str) -> Token<'src> {
        Token { literal, kind: Error, line: self.line }
    }

     fn consume(&mut self) -> &str {
        self.current += 1;
        &self.source[self.current - 1 .. self.current]
    }

    fn peek(&self) -> &str {
        if self.is_at_end() { // TODO: fix this hack
            "\0"
        } else {
            &self.source[self.current .. self.current + 1]
        }
    }

    fn peek_forward(&self) -> &str {
        if self.is_at_end() {
            "\0"
        } else {
            &self.source[self.current + 1 .. self.current + 2]
        }
    }

    fn peek_back(&self) -> &str {
        &self.source[self.current - 1 .. self.current]
    }

    fn matches(&mut self, expected: &str) -> bool {
        let consumed = !self.is_at_end() && self.peek() == expected;
        if consumed { self.current += 1 }
        consumed
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn is_digit(s: &str) -> bool {
    char(s).is_ascii_digit()
}

fn is_alpha(s: &str) -> bool {
    matches!(char(s), 'a'..='z' | 'A'..='Z' | '_')
}