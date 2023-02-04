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

use xlqs::{ConnectionPool, Error};
use entity::{Task, Status::*};

#[main]
async fn main() -> Result<(), Error> {
    let pool = ConnectionPool::new(
        "mysql://user:password@localhost:5432/sample"
    ).await?;

    Task::INSERT(&pool)
        .VALUES(|t| t
            .subject("task1")
            .status(Ready)
        )
        .await?;
    
    let task = Task::ONLY(&pool)
        .WHERE(|t| t
            .subject_like("$task")
            .status_eq(Ready)
        )
        .await?;

    let first_task = Task::FIRST(&pool)
        .WHERE(|t| t
            .subject_like("$task")
            .status_eq(Ready)
        )
        .await?;

    let ready_tasks = Task::ALL(&pool)
        .WHERE(|t| t.status_eq(Ready))
        .ORDER_DESC(|t| t.id)
        .LIMIT(100)
        .await?;

    Task::UPDATE(&pool)
        .SET(|t| t.status(Ready))
        .WHERE(|t| t.subject_eq("task1"))
        .await?;

    Task::DELETE(&pool)
        .WHERE(|t| t
            .subject_like("$task")
            .status_eq(Completed)
        )
        .await?;
}
```
