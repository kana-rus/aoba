use super::Condition;

pub struct StringCondition<const COLUMN: &'static str> {
    pub NOT: StringNegativeCondition<COLUMN>,
    content: String,
}
pub struct StringNegativeCondition<const COLUMN: &'static str> {
    content: String,
}

impl<const COLUMN: &'static str> StringCondition<COLUMN> {
    pub fn new() -> Self {
        Self {
            NOT: StringNegativeCondition { content: String::new() },
            content: String::new(),
        }
    }
}

pub trait AsStr {fn as_str(&self) -> &str;}
impl AsStr for String {fn as_str(&self) -> &str {&self}}
impl AsStr for &str {fn as_str(&self) -> &str {self}}

impl<const COLUMN: &'static str> StringCondition<COLUMN> {
    pub fn equals<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} = '{}'", another.as_str()))
    }
    pub fn greater_than<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} > '{}'", another.as_str()))
    }
    pub fn less_than<S: AsStr>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} < '{}'", another.as_str()))
    }
}
