#![allow(non_camel_case_types)]

/// Internal `&str` is automatically generated from input tokens, so you can input literally **any**thing in `()`.
/// ```no_run
/// schema!{
///     User {
///         name: TEXT where DEFAULT("No name"),
///         age:  SMALLINT where DEFAULT(42)
///          // Of course this is valid  ↑↑
///     }
/// }
/// ```
pub struct any(pub &'static str);

pub use db::{Mixin, DBType, ColumnConstrain, TableConstrain};

#[cfg(feature="postgres")]
mod postgres;
#[cfg(feature="postgres")]
use postgres as db;

#[cfg(feature="mysql")]
mod mysql;
#[cfg(feature="mysql")]
use mysql as db;
