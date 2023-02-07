use futures_core::stream::BoxStream;
use sqlx::{Database, Executor, FromRow, Row};

pub enum Order {
    Asc,
    Desc,
}

#[allow(non_camel_case_types)]
pub struct string(
    std::borrow::Cow<'static, str>
); impl string {
    pub fn as_str(&self) -> &str {
        &&self.0
    }
}
impl Into<string> for String {
    fn into(self) -> string {
        string(std::borrow::Cow::Owned(self))
    }
}
impl Into<string> for &String {
    fn into(self) -> string {
        string(std::borrow::Cow::Owned(self.to_owned()))
    }
}
impl Into<string> for &'static str {
    fn into(self) -> string {
        string(std::borrow::Cow::Borrowed(self))
    }
}

pub trait Query: Sized {
    type DB: Database;

    fn exec<'e, E: Executor<'e>>(
        self,
        executor: E,
    ) -> BoxStream<'e, sqlx::Result<<Self::DB as Database>::QueryResult>>;

    fn save<'e, 'r, As: FromRow<'r, R>, R: Row, E: Executor<'e>>(
        self,
        executor: E,
    ) -> BoxStream<'e, sqlx::Result<As>>;
}
