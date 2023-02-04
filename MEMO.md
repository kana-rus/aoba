```rust
use lqs::{Entity, Text, DateTime};

#[derive(Entity)]
// #[table("tasks")]             <-- default behavior
pub struct Task {
    #[primary_key]   pub id:      u64,
    #[db_type(Text)] pub subject: String,
    pub status:                   Status,
}

#[derive(Entity)]
pub enum Status {
    // #[db_value("ready")]       <-- default behavior
    Ready,
    // #[db_value("completed")]   <-- default behavior
    Completed,
}
```

```rust
mod entity;

use lqs::{ConnectionPool, Error};
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
