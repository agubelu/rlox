/** Big declaration chunks that I didn't wanna have around actual code logic. */

use crate::scanning::TokenType::{self, *};
use super::Parser;
use Precedence::*;

pub type ParseFn<'src, 'chk> = Option<fn(&mut Parser<'src, 'chk>)>;
pub type ParseRule<'src, 'chk> = (ParseFn<'src, 'chk>, ParseFn<'src, 'chk>, Precedence);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    // This relies on implicit discriminant value assignment:
    // the first value is 0, and all values after are 1 higher
    // than the previous one. Precedence is thus declared
    // from lowest to highest.
    NoPr,       // Renamed from None to avoid clashing with Option::None
    Assign,     // =
    BoolOr,     // ||
    BoolAnd,    // &&
    Equality,   // == !=
    Compare,    // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // - !
    Call,       // () .
    Primary,    // Literals
}

impl Precedence {
    pub fn higher(&self) -> Self {
        // TODO: possible optimization
        match self {
            NoPr => Assign,
            Assign => BoolOr,
            BoolOr => BoolAnd,
            BoolAnd => Equality,
            Equality => Compare,
            Compare => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Primary,
            Primary => Primary,
        }
    }
}

impl<'src, 'chk> Parser<'src, 'chk> {
    pub(super) fn get_rule(kind: TokenType) -> ParseRule<'src, 'chk> {
        match kind {
            LeftParen => (Some(Self::grouping), None, NoPr),
            Minus => (Some(Self::unary), Some(Self::binary), Term),
            Plus => (None, Some(Self::binary), Term),
            Slash => (None, Some(Self::binary), Factor),
            Asterisk => (None, Some(Self::binary), Factor),
            Number => (Some(Self::number), None, NoPr),
            Null => (Some(Self::literal), None, NoPr),
            True => (Some(Self::literal), None, NoPr),
            False => (Some(Self::literal), None, NoPr),
            Not => (Some(Self::unary), None, NoPr),
            _ => (None, None, NoPr)
        }
    }
}
