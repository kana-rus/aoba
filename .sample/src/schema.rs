use xlqs::{entity, db_types::{TIMESTAMP, SERIAL, CHAR, TEXT}};

entity! {
    #[with(id, time)]
    User {
        name:     CHAR(20),
        password: CHAR(20),
    },
    #[with(id, time)]
    Task {
        user_id *-> User.id,
        title:      CHAR(20),
        body:       TEXT,
    },
}

// ==========
// expanded:

pub(crate) struct User {
    id: Result<u32>,
    name: Result<String>,
    password: Result<String>,
    created_at: Result<DateTime>,
    updated_at: Result<DateTime>,
}

pub(crate) struct Task {
    id: Result<u32>,
    user_id: Result<u32>,
    title: Result<String>,
    body: Result<String>,
    created_at: Result<DateTime>,
    updated_at: Result<DateTime>,
}

// ==========
/*
    $ xlqs migrate (= `xlqs generate && xlqs migrate`)
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
