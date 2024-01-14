use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub enum Object {
    String(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
}