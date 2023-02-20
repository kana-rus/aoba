use std::fmt::Display;

mod number; pub use number::NumberCondition;
mod string; pub use string::StringCondition;

pub struct Condition(String);
impl Condition {
    #[inline] pub fn OR(self, another: Self) -> Condition {
        Condition(format!(
            "({} OR {})", self.0, another.0
        ))
    }
    #[inline] pub fn new() -> Self {
        Self(format!("WHERE"))
    }
}
impl Condition {
    // #[inline] fn is_empty(&self) -> bool {
    //     self.0.len() == 5
    // }
    #[inline] fn AND(self, another: Self) -> Condition {
        Condition(format!(
            "({} AND {})", self.0, another.0
        ))
    }
}

impl<const N: usize> Into<Condition> for [Condition; N] {
    #[inline] fn into(self) -> Condition {
        self.into_iter().fold(Condition::new(), |it, next| it.AND(next))
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
