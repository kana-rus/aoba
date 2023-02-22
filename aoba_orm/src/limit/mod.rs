use std::fmt::Display;

pub struct Limit(Option<usize>);
impl Limit {
    #[inline] pub fn new() -> Self {
        Self(None)
    }
    #[inline] pub fn set(&mut self, limit: usize) {
        self.0.replace(limit);
    }
}

impl Display for Limit {
    #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, ""),
            Some(limit) => write!(f, "LIMIT {limit}"),
        }
    }
}
