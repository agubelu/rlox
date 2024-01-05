/** Big declaration chunks that I didn't wanna have around actual code logic. */
use super::Parser;
use Precedence::*;

pub type ParseFn<'a, 'b> = Option<fn(&mut Parser<'a, 'b>) -> ()>;
pub type ParseRule<'a, 'b> = (ParseFn<'a, 'b>, ParseFn<'a, 'b>, Precedence);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    // This relies on implicit discriminant value assignment:
    // the first value is 0, and all values after are 1 higher
    // than the previous one. Precedence is thus declared
    // from lowest to highest.
    NoPr,       // Renamed from None to avoid clashing with Option::None
    Assign,     // =
    Or,         // ||
    And,        // &&
    Equality,   // == !==
    Compare,    // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // - !
    Call,       // () .
    Primary,    // Literals
}

pub static RulesTable: [ParseRule; 1] = [
    (None, Some(Parser::number), NoPr)
];