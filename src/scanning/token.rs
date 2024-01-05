use TokenType::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TokenType {
    // Single-characters
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Asterisk,

    // Boolean operators
    Not, NotEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, If, Else, True, False,
    For, Fn, Null, Or, Print, Return,
    Super, This, Let, While,

    // Aux
    Eof,
    #[default]
    Error,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Token<'src> {
    pub kind: TokenType,
    pub line: u32,
    pub literal: &'src str,
}

impl TokenType {
    pub fn from_literal(literal: &str) -> Self {
        match literal {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fn" => Fn,
            "if" => If,
            "let" => Let,
            "null" => Null,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "while" => While,
            _ => Identifier,
        }
    }
}