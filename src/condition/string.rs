use super::Condition;

pub struct StringCondition<const COLUMN: &'static str>
;
// {
//     pub NOT: StringNegativeCondition<COLUMN>
// }
// pub struct StringNegativeCondition<const COLUMN: &'static str>;
// 
// impl<const COLUMN: &'static str> StringCondition<COLUMN> {
//     pub fn new() -> Self {
//         Self { NOT: StringNegativeCondition }
//     }
// }

pub trait AsStr {fn as_str(&self) -> &str;}
impl AsStr for String {fn as_str(&self) -> &str {&self}}
impl AsStr for &str {fn as_str(&self) -> &str {self}}

impl<const COLUMN: &'static str> StringCondition<COLUMN> {
    pub fn eq<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} = '{}'", another.as_str()))
    }
    pub fn like<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} LIKE '{}'", another.as_str()))
    }

    pub fn ne<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("NOT {COLUMN} = '{}'", another.as_str()))
    }
    pub fn unlike<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("NOT {COLUMN} LIKE '{}'", another.as_str()))
    }
// }
// impl<const COLUMN: &'static str> StringNegativeCondition<COLUMN> {
//     pub fn eq<S: AsStr>(&self, another: S) -> Condition {
//         Condition(format!("NOT {COLUMN} = '{}'", another.as_str()))
//     }
//     pub fn like<S: AsStr>(&self, another: S) -> Condition {
//         Condition(format!("NOT {COLUMN} LIKE '{}'", another.as_str()))
//     }
}
