#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
// #![feature(generic_arg_infer)]

// === x ===
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
// =========

mod connection_pool;
mod entity;
mod experiment;

// use sqlx::{Pool, Database};
// 
// impl<DB: Database> ConnectionPool for Pool<DB> {
// 
// }
