pub enum Order {
    Asc(&'static str),
    Desc(&'static str),
}

pub trait AsStr {fn as_str(&self) -> &str;}
impl AsStr for String {fn as_str<'s>(&self) -> &str {&self}}
impl AsStr for &str {fn as_str(&self) -> &str {self}}

pub trait AsString {fn as_string(self) -> String;}
impl AsString for String {fn as_string(self) -> String {self}}
impl AsString for &str {fn as_string(self) -> String {self.to_owned()}}
