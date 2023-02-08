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

pub trait Query<'e, E: Executor<'e>> {
    type Return: for <'r> FromRow<'r, <<E as Executor<'e>>::Database as sqlx::Database>::Row>;

    async fn exec(self, executer: E) -> Result<QuerySucceed, Error>;
    async fn save(self, executer: E) -> Result<<Self as Query<'e, E>>::Return, Error>;
}

pub struct QuerySucceed;
impl<'r, R: Row> FromRow<'r, R> for QuerySucceed {
    #[inline] fn from_row(_: &'r R) -> Result<Self, Error> {
        Ok(QuerySucceed)
    }
}
