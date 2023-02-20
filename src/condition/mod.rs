mod number; pub use number::{NumberCondition, NumberNegativeCondition};
mod string; pub use string::{StringCondition, StringNegativeCondition};

pub struct Condition(String);
impl Condition {
    #[inline] fn is_empty(&self) -> bool {
        self.0.len() == 5
    }
    #[inline] fn AND(self, another: Self) -> Condition {
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
        Self(format!("WHERE"))
    }
}

impl<const N: usize> Into<Condition> for [Condition; N] {
    fn into(self) -> Condition {
        self.into_iter().fold(Condition::new(), |it, next| it.AND(next))
    }
}
