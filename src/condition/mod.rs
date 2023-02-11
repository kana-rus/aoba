use std::marker::PhantomData;

pub struct Condition(String);
impl Condition {
    fn AND(self, another: Self) -> Condition {
        Condition(format!(
            "({} AND {})", self.0, another.0
        ))
    }
    fn OR(self, another: Self) -> Condition {
        Condition(format!(
            "({} OR {})", self.0, another.0
        ))
    }
}

pub struct ConditionBuilder<const COLUMN: &'static str, Type> {
    pub NOT: NegativeConditionBuilder<COLUMN, Type>,
    r#type:  PhantomData<Type>,
    content: String,
}
pub struct NegativeConditionBuilder<const COLUMN: &'static str, Type> {
    r#type:  PhantomData<Type>,
    content: String,
}


impl<const COLUMN: &'static str> ConditionBuilder<COLUMN, i64> {
    pub fn equals(&self, another: i64) -> Condition {
        Condition(format!("{COLUMN} = {another}"))
    }
    pub fn greater_than(&self, another: i64) -> Condition {
        Condition(format!("{COLUMN} > {another}"))
    }
    pub fn less_than(&self, another: i64) -> Condition {
        Condition(format!("{COLUMN} < {another}"))
    }
}
