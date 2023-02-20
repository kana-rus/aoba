use super::Condition;
use std::fmt::Display;

pub struct NumberCondition<const COLUMN: &'static str> {
    pub NOT: NumberNegativeCondition<COLUMN>,
    content: String,
}
pub struct NumberNegativeCondition<const COLUMN: &'static str> {
    content: String,
}

impl<const COLUMN: &'static str> NumberCondition<COLUMN> {
    pub fn new() -> Self {
        Self {
            NOT: NumberNegativeCondition { content: String::new() },
            content: String::new(),
        }
    }
}

pub trait Number: Display {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}

impl<const COLUMN: &'static str> NumberCondition<COLUMN> {
    pub fn equals<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} = {another}"))
    }
    pub fn greater_than<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} > {another}"))
    }
    pub fn less_than<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} < {another}"))
    }
}
impl<const COLUMN: &'static str> NumberNegativeCondition<COLUMN> {
    pub fn equals<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} = {another}"))
    }
    pub fn greater_than<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} > {another}"))
    }
    pub fn less_than<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} < {another}"))
    }
}
