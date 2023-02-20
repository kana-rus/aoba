use std::fmt::Display;

pub struct OrderBy(String);
#[allow(non_snake_case)]
impl OrderBy {
    #[inline] pub fn new() -> Self {
        Self(String::new())
    }
    #[inline] pub fn ASC(&mut self, by: &'static str) {
        self.0 += " ORDER BY ";
        self.0 += by;
    }
    #[inline] pub fn DESC(&mut self, by: &'static str) {
        self.0 += " ORDER BY ";
        self.0 += by;
        self.0 += " DESC";
    }
}

impl Display for OrderBy {
    #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
