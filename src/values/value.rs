use std::fmt::{Debug, Display};
use std::ops::{Add, Sub, Mul, Div, Neg, Not, BitAnd, BitOr};
use crate::Object;
use LoxValue::*;

#[derive(Clone, PartialEq)]
pub enum LoxValue {
    /* Important note: the various binary operations implemented
    for LoxValue put the right-hand operand on the left. This is
    because a binary expression like `1 - 2` first pops the
    right-side operand from the stack as it was computed last,
    and then the left-side one. So, the operation methods
    are actually called on the right-side operand, and the expression
    `1 - 2` would turn into `2.sub(1)`, which needs to internally
    flip the operands to return the correct value. */
    Bool(bool),
    Number(f64),
    Object(Object),
    Null,
}

impl LoxValue {
    pub fn is_falsey(&self) -> bool {
        matches!(self, Null | Bool(false))
    }

    pub fn less(&self, rhs: &Self) -> Option<Self> {
        if let (Number(a), Number(b)) = (rhs, self) {
            Some(Bool(a < b))
        } else {
            None
        }
    }

    pub fn greater(&self, rhs: &Self) -> Option<Self> {
        if let (Number(a), Number(b)) = (rhs, self) {
            Some(Bool(a > b))
        } else {
            None
        }
    }
}

impl Eq for LoxValue {}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bool(b) => write!(f, "{b}"),
            Number(n) => write!(f, "{n}"),
            Object(o) => write!(f, "{o}"),
            Null => write!(f, "null"),
        }
    }
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Add<Self> for LoxValue {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (rhs, self) {
            (Number(a), Number(b)) => Some(Number(a + b)),
            (Object(Object::String(a)), Object(Object::String(b))) => {
                Some(Object(Object::String(a + &b)))
            },
            _ => None,
        }
    }
}

impl Sub<Self> for LoxValue {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let (Number(a), Number(b)) = (rhs, self) {
            Some(Number(a - b))
        } else {
            None
        }
    }
}

impl Mul<Self> for LoxValue {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let (Number(a), Number(b)) = (rhs, self) {
            Some(Number(a * b))
        } else {
            None
        }
    }
}

impl Div<Self> for LoxValue {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        if let (Number(a), Number(b)) = (rhs, self) {
            Some(Number(a / b))
        } else {
            None
        }
    }
}

impl Neg for LoxValue {
    type Output = Option<Self>;

    fn neg(self) -> Self::Output {
        if let Number(a) = self {
            Some(Number(-a))
        } else {
            None
        }
    }
}

impl Not for LoxValue {
    type Output = Option<Self>;

    fn not(self) -> Self::Output {
        if let Bool(b) = self {
            Some(Bool(!b))
        } else {
            None
        }
    }
}

impl BitAnd for LoxValue {
    type Output = Option<Self>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if let (Bool(a), Bool(b)) = (self, rhs) {
            Some(Bool(a && b))
        } else {
            None
        }
    }
}

impl BitOr for LoxValue {
    type Output = Option<Self>;

    fn bitor(self, rhs: Self) -> Self::Output {
        if let (Bool(a), Bool(b)) = (self, rhs) {
            Some(Bool(a || b))
        } else {
            None
        }
    }
}
