use sqlx::{Executor, FromRow, Row, Error};

pub enum Order {
    Asc,
    Desc,
}


// #[allow(non_camel_case_types)]
// pub struct string(
//     std::borrow::Cow<'static, str>
// ); impl string {
//     pub fn as_str(&self) -> &str {
//         &&self.0
//     }
// }
// impl Into<string> for String {
//     fn into(self) -> string {
//         string(std::borrow::Cow::Owned(self))
//     }
// }
// impl Into<string> for &String {
//     fn into(self) -> string {
//         string(std::borrow::Cow::Owned(self.to_owned()))
//     }
// }
// impl Into<string> for &'static str {
//     fn into(self) -> string {
//         string(std::borrow::Cow::Borrowed(self))
//     }
// }
#[allow(non_camel_case_types)]
pub trait string {
    fn to_string(self) -> String;
    fn as_str(&self) -> &str;
}
impl string for String {
    fn to_string(self) -> String {self}
    fn as_str(&self) -> &str {&self}
}
impl string for &str {
    fn to_string(self) -> String {self.to_owned()}
    fn as_str(&self) -> &str {self}
}

pub trait Exec<'r, R: Row> {
    type Return: FromRow<'r, R>;
    async fn exec<'e, E: Executor<'e>>(self, executer: E) -> Result<<Self as Exec<'r, R>>::Return, Error>;
}

pub struct QuerySucceed;
impl<'r, R: Row> FromRow<'r, R> for QuerySucceed {
    #[inline] fn from_row(_: &'r R) -> Result<Self, Error> {
        Ok(QuerySucceed)
    }
}
