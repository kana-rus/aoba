use std::borrow::Cow;

pub enum Order {
    Asc,
    Desc,
}

#[allow(non_camel_case_types)]
pub struct string(
    Cow<'static, str>
); impl string {
    pub fn as_str(&self) -> &str {
        &&self.0
    }
}
impl Into<string> for String {
    fn into(self) -> string {
        string(Cow::Owned(self))
    }
}
impl Into<string> for &String {
    fn into(self) -> string {
        string(Cow::Owned(self.to_owned()))
    }
}
impl Into<string> for &'static str {
    fn into(self) -> string {
        string(Cow::Borrowed(self))
    }
}
