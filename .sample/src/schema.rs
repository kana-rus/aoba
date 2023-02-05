use xlqs::{entity, db_types::{TIMESTAMP, SERIAL, CHAR, TEXT}};

entity! {
    #[with(time)]
    User {
        // #[PRIMARY_KEY]
        // id:       SERIAL,
        name:     CHAR(20),
        password: CHAR(20),
    },
    #[with(time)]
    Task {
        // #[PRIMARY_KEY]
        // id:         SERIAL,
        user_id *-> User.id,
        title:      CHAR(20),
        body:       TEXT,
    },
}

// ==========
// expanded:

struct UserEntity {
    id: Result<u32>,
    name: Result<String>,
    password: Result<String>,
    created_at: Result<DateTime>,
    updated_at: Result<DateTime>,
}
struct User {
    id: u32,
    name: String,
    password: String,
}
impl UserEntity {

}

struct TaskEntity {
    id: Result<u32>,
    user_id: Result<u32>,
    title: Result<String>,
    body: Result<String>,
    created_at: Result<DateTime>,
    updated_at: Result<DateTime>,
}
struct Task {
    id: u32,
    user_id: u32,
    title: String,
    body: String,
}

impl Entity {
    pub fn as_user(self) -> Result<User> {
        match self {
            Self::User {
                id,
                name,
                password,
            } => Ok(User {
                id: id.ok_or_else(|| ..)?,
                name: name.ok_or_else(|| ..)?,
                password: password.ok_or_else(|| ..)?,
            }),
            _ => unreachable!()
        }
    }
    pub fn as_task(self) -> Result<Task> {
        match self {
            Self::Task {
                id,
                user_id,
                title,
                body,
            } => Ok(Task {
                id: id.ok_or_else(|| ..)?,
                user_id: user_id.ok_or_else(|| ..)?,
                title: title.ok_or_else(|| ..)?,
                body: body.ok_or_else(|| ..)?,
            }),
            _ => unreachable!()
        }
    }
}

// ==========
/*
    $ xlqs migrate
      --url mysql://user:password@localhost:5432/sample
      [ --schema src/schema.rs (: default behavior) ]
      [ --out ./migration/sql  (: default behavior) ]

*//* generated: schema.sql

CREATE TABLE users (
    id SERIAL,
    name CHAR(20),
    password CHAR(20),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,

    PRIMARY KEY (id)
)

CREATE TABLE tasks (
    id SERIAL,
    user_id SERIAL,
    title CHAR(20),
    body CHAR(20),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,

    PRIMARY KEY (id),

    FOREIGN KEY (user_id) REFERENCES users (id),
)

*/
