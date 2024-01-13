use std::fmt::{Debug, Display};
use std::ops::{Add, Sub, Mul, Div, Neg, Not, BitAnd, BitOr};

#[derive(Copy, Clone, PartialEq)]
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
    Null,
}

impl LoxValue {
    pub fn is_falsey(&self) -> bool {
        match self {
            Self::Null => true,
            Self::Bool(b) => !b,
            Self::Number(n) => *n == 0.0,
        }
    }

    pub fn less(&self, rhs: &Self) -> Option<Self> {
        if let (Self::Number(a), Self::Number(b)) = (rhs, self) {
            Some(Self::Bool(a < b))
        } else {
            None
        }
    }

    pub fn greater(&self, rhs: &Self) -> Option<Self> {
        if let (Self::Number(a), Self::Number(b)) = (rhs, self) {
            Some(Self::Bool(a > b))
        } else {
            None
        }
    }
}

impl Eq for LoxValue {}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Bool(b) => write!(f, "{b}"),
            LoxValue::Number(n) => write!(f, "{n}"),
            LoxValue::Null => write!(f, "null"),
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
        if let (Self::Number(a), Self::Number(b)) = (rhs, self) {
            Some(Self::Number(a + b))
        } else {
            None
        }
    }
}

impl Sub<Self> for LoxValue {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let (Self::Number(a), Self::Number(b)) = (rhs, self) {
            Some(Self::Number(a - b))
        } else {
            None
        }
    }
}

impl Mul<Self> for LoxValue {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let (Self::Number(a), Self::Number(b)) = (rhs, self) {
            Some(Self::Number(a * b))
        } else {
            None
        }
    }
}

impl Div<Self> for LoxValue {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        if let (Self::Number(a), Self::Number(b)) = (rhs, self) {
            Some(Self::Number(a / b))
        } else {
            None
        }
    }
}

impl Neg for LoxValue {
    type Output = Option<Self>;

    fn neg(self) -> Self::Output {
        if let Self::Number(a) = self {
            Some(Self::Number(-a))
        } else {
            None
        }
    }
}

impl Not for LoxValue {
    type Output = Option<Self>;

    fn not(self) -> Self::Output {
        if let Self::Bool(b) = self {
            Some(Self::Bool(!b))
        } else {
            None
        }
    }
}

impl BitAnd for LoxValue {
    type Output = Option<Self>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if let (Self::Bool(a), Self::Bool(b)) = (self, rhs) {
            Some(Self::Bool(a && b))
        } else {
            None
        }
    }
}

impl BitOr for LoxValue {
    type Output = Option<Self>;

    fn bitor(self, rhs: Self) -> Self::Output {
        if let (Self::Bool(a), Self::Bool(b)) = (self, rhs) {
            Some(Self::Bool(a || b))
        } else {
            None
        }
    }
}
