/** Big declaration chunks that I didn't wanna have around actual code logic. */

use crate::scanning::TokenType::{self, *};
use super::Parser;

pub type ParseFn<'src, 'chk> = Option<fn(&mut Parser<'src, 'chk>)>;
pub type ParseRule<'src, 'chk> = (ParseFn<'src, 'chk>, ParseFn<'src, 'chk>, Precedence);

pub type Precedence = u8;
pub struct Precs;
impl Precs {
    pub const NONE: Precedence = 0;
    pub const ASSIGN: Precedence = 1;   // =
    pub const BOOL_OR: Precedence = 2;  // ||
    pub const BOOL_AND: Precedence = 3; // &&
    pub const EQUALITY: Precedence = 4; // == !=
    pub const COMPARE: Precedence = 5;  // < <= > >=
    pub const TERM: Precedence = 6;     // + -
    pub const FACTOR: Precedence = 7;   // * /
    pub const UNARY: Precedence = 8;    // - !
    pub const CALL: Precedence = 9;     // () .
    pub const PRIMARY: Precedence = 10; // Literals
}

impl<'src, 'chk> Parser<'src, 'chk> {
    pub(super) fn get_rule(kind: TokenType) -> ParseRule<'src, 'chk> {
        match kind {
            LeftParen => (
                Some(Self::grouping),
                None,
                Precs::NONE
            ),
            Minus => (
                Some(Self::unary),
                Some(Self::binary),
                Precs::TERM
            ),
            Plus => (
                None,
                Some(Self::binary),
                Precs::TERM
            ),
            Slash => (
                None,
                Some(Self::binary),
                Precs::FACTOR
            ),
            Asterisk => (
                None,
                Some(Self::binary),
                Precs::FACTOR
            ),
            Number => (
                Some(Self::number),
                None,
                Precs::NONE
            ),
            Null => (
                Some(Self::literal),
                None,
                Precs::NONE
            ),
            True => (
                Some(Self::literal),
                None,
                Precs::NONE
            ),
            False => (
                Some(Self::literal),
                None,
                Precs::NONE
            ),
            Not => (
                Some(Self::unary),
                None,
                Precs::NONE
            ),
            _ => (None, None, Precs::NONE)
        }
    }
}
