use std::fmt::Display;
use std::ops::{BitOr, BitAnd};

mod number; pub use number::NumberCondition;
mod string; pub use string::{StringCondition, Str};

pub struct Condition(String);
impl Condition {
    #[inline] pub fn AND(self, another: Self) -> Condition {
        Condition(format!(
            "({} AND {})", self.0, another.0
        ))
    }
    #[inline] pub fn OR(self, another: Self) -> Condition {
        Condition(format!(
            "({} OR {})", self.0, another.0
        ))
    }
    #[inline] pub fn new() -> Self {
        Self(String::new())
    }
}
impl BitOr for Condition {
    type Output = Condition;
    #[inline] fn bitor(self, rhs: Self) -> Self::Output {
        self.OR(rhs)
    }
}
impl BitAnd for Condition {
    type Output = Condition;
    #[inline] fn bitand(self, rhs: Self) -> Self::Output {
        self.AND(rhs)
    }
}

impl<const N: usize> Into<Condition> for [Condition; N] {
    #[inline] fn into(self) -> Condition {
        self.into_iter().fold(Condition::new(), |it, next| it.AND(next))
    }
}

impl Display for Condition {
    #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {write!(f, "")} else {write!(f, "WHERE {}", self.0)}
    }
}
