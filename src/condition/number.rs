use std::fmt::Display;
use super::Condition;

pub struct NumberCondition<const COLUMN: &'static str>
;
// {
//     pub NOT: NumberNegativeCondition<COLUMN>
// }
// pub struct NumberNegativeCondition<const COLUMN: &'static str>;
// 
// impl<const COLUMN: &'static str> NumberCondition<COLUMN> {
//     pub fn new() -> Self {
//         Self { NOT: NumberNegativeCondition }
//     }
// }

pub trait Number: Display {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}

impl<const COLUMN: &'static str> NumberCondition<COLUMN> {
    pub fn eq<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} = {another}"))
    }
    pub fn gt<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} > {another}"))
    }
    pub fn lt<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} < {another}"))
    }

    pub fn ne<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} = {another}"))
    }
    pub fn ge<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} < {another}"))
    }
    pub fn le<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} > {another}"))
    }
// }
// 
// impl<const COLUMN: &'static str> NumberNegativeCondition<COLUMN> {
//     pub fn eq<N: Number>(&self, another: N) -> Condition {
//         Condition(format!("NOT {COLUMN} = {another}"))
//     }
//     pub fn gt<N: Number>(&self, another: N) -> Condition {
//         Condition(format!("NOT {COLUMN} > {another}"))
//     }
//     pub fn lt<N: Number>(&self, another: N) -> Condition {
//         Condition(format!("NOT {COLUMN} < {another}"))
//     }
}
