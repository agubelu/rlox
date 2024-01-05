/** Big declaration chunks that I didn't wanna have around actual code logic. */
use super::Parser;
use Precedence::*;

pub type ParseFn<'src, 'chk> = for<'a> fn(&'a mut Parser<'src, 'chk>);
// pub type ParseFnOpt = Option<ParseFn>;
pub type ParseRule<'src, 'chk> = (ParseFn<'src, 'chk>, ParseFn<'src, 'chk>, Precedence);

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

impl Precedence {
    pub fn higher(&self) -> Self {
        // TODO: possible optimization
        match self {
            NoPr => Assign,
            Assign => Or,
            Or => And,
            And => Equality,
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

pub static RulesTable: [ParseRule; 1] = [
    // Note: the order of this table is highly dependent on the
    // order of the token types in scanning::token::TokenType
    (Parser::number, Parser::number, NoPr)
];