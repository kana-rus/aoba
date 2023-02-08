#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

// === x ===
#![feature(generic_const_exprs)]
// =========

mod connection_pool;
mod entity;
mod experiment;

// use sqlx::{Pool, Database};
// 
// impl<DB: Database> ConnectionPool for Pool<DB> {
// 
// }
