```rust
use musql::{Entity, Text, DateTime};

#[derive(Entity)]
// #[table("tasks")]           <-- default behavior
pub struct Task {
    #[primary_key] id: u64,
    subject:           String,
    status:            Status,
}

#[derive(Entity)]
pub enum Status {
    // #[value("ready")]       <-- default behavior
    Ready,
    // #[value("completed")]   <-- default behavior
    Completed,
}

---
mod entity;

use musql::{ConnectionPool, Error};
use entity::{Task, Status::*};

#[main]
async fn main() -> Result<(), Error> {
    let pool = ConnectionPool::new(
        "mysql://user:password@localhost:5432/sample"
    ).await?;

    pool.INSERT(
        Task::VALUES()
            .subject("task1")
            .status(Ready)
    ).await?;

    let task = pool.SELECT_UNIQUE(
        Task::WHERE()
            .subject_like("$task")
            .status_eq(Ready)
    ).await?;

    pool.UPDATE(
        Task::SET().status(Ready)
            .WHERE().subject_eq("task1")
    ).await?;

    pool.DELETE(
        Task::WHERE()
            .subject_like("$task")
            .status_eq(Completed)
    ).await?;
}
```
