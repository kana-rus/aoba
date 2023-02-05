use aoba::{
    schema,
    DateTime,
    mixins::{with, id, times},
    db_type::{CHAR, TEXT, reference},
};

schema! {
    #[with(id, times)]
    User {
        name:     CHAR(20),
        password: CHAR(20),
    },
    #[with(id, times)]
    Task {
        user_id: reference(User.id),
        title:   CHAR(20),
        body:    TEXT,
    },
}

// ==========
// expanded:

pub(crate) struct User {
    id: Option<u32>,
    name: Option<String>,
    password: Option<String>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}
#[allow(non_snake_case)]
impl User {
    pub fn FIRST<E: sqlx::Executor>(executer: E) -> UserSelecter {
        UserSelecter {
            executer,
            select1: true,
            condition: UserCondition::default()
        }
    }
    pub fn ALL<E: sqlx::Executor>(executer: E) -> UserSelecter {
        UserSelecter {
            executer,
            select1: false,
            condition: UserCondition::default()
        }
    }

    pub fn UPDATE<E: sqlx::Executor>(executer: E) -> UserUpdater {
        UserUpdater {
            executer,
            set_values: User {
                id: None,
                name: None,
                password: None,
                created_at: None,
                updated_at: None,
            },
            condition: UserCondition::default()
        }
    }
    pub fn DELETE<E: sqlx::Executor>(executer: E) -> UserDeleter {
        UserDeleter {
            executer,
            condition: UserCondition::default()
        }
    }
}
pub(crate) struct CreateUser {
    pub(crate) name: String,
    pub(crate) password: String,
}

struct UserFields {
    id: UserField,
    name: UserField,
    password: UserField,
    created_at: UserField,
    updated_at: UserField,
}
#[allow(non_camel_case_types)]
enum UserField {id, name, password, created_at, updated_at}
impl UserField {
    fn as_str(self) -> &'static str {
        match self {
            Self::id => "id",
            Self::name => "name",
            Self::password => "password",
            Self::created_at => "created_at",
            Self::updated_at => "updated_at",
        }
    }
}

struct UserSelecter<E: sqlx::Executor> {
    executer: E,
    select1: bool,
    limit: Option<usize>,
    order: Option<aoba::Order>,
    condition: UserCondition,
} impl UserSelecter {
    pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
        self.condition = condition_builder(self.condition);
        self
    }
    pub fn LIMIT(mut self, limit: usize) -> usize {
        self.limit = Some(limit);
        self
    }
    pub fn ORDER_ASC<F: Fn(UserFields) -> UserField>(mut self, column_selecter: F) -> Self {
        let field = column_selecter(UserFields {
            id: UserField::id,
            name: UserField::name,
            password: UserField::password,
            created_at: UserField::created_at,
            updated_at: UserField::updated_at,
        });
        self.order = Some(aoba::Order::Asc(field.as_str()));
        self
    }
    pub fn ORDER_DESC<F: Fn(UserFields) -> UserField>(mut self, column_selecter: F) -> Self {
        let field = column_selecter(UserFields {
            id: UserField::id,
            name: UserField::name,
            password: UserField::password,
            created_at: UserField::created_at,
            updated_at: UserField::updated_at,
        });
        self.order = Some(aoba::Order::Desc(field.as_str()));
        self
    }
}
struct UserUpdater<E: sqlx::Executor> {
    executer: E,
    limit: Option<usize>,
    order: Option<aoba::Order>,
    set_values: User,
    condition: UserCondition,
} impl UserUpdater {
    pub fn VALUES<F: Fn(User) -> User>(mut self, values_setter: F) -> Self {
        self.set_values = values_setter(self.set_values);
        self
    }

    pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
        self.condition = condition_builder(self.condition);
        self
    }
    pub fn LIMIT(mut self, limit: usize) -> usize {
        self.limit = Some(limit);
        self
    }
    pub fn ORDER_ASC<F: Fn(UserFields) -> UserField>(mut self, column_selecter: F) -> Self {
        let field = column_selecter(UserFields {
            id: UserField::id,
            name: UserField::name,
            password: UserField::password,
            created_at: UserField::created_at,
            updated_at: UserField::updated_at,
        });
        self.order = Some(aoba::Order::Asc(field.as_str()));
        self
    }
    pub fn ORDER_DESC<F: Fn(UserFields) -> UserField>(mut self, column_selecter: F) -> Self {
        let field = column_selecter(UserFields {
            id: UserField::id,
            name: UserField::name,
            password: UserField::password,
            created_at: UserField::created_at,
            updated_at: UserField::updated_at,
        });
        self.order = Some(aoba::Order::Desc(field.as_str()));
        self
    }
}
struct UserDeleter<E: sqlx::Executor> {
    executer: E,
    limit: Option<usize>,
    order: Option<aoba::Order>,
    condition: UserCondition,
} impl UserDeleter {
    pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
        self.condition = condition_builder(self.condition);
        self
    }
    pub fn LIMIT(mut self, limit: usize) -> usize {
        self.limit = Some(limit);
        self
    }
    pub fn ORDER_ASC<F: Fn(UserFields) -> UserField>(mut self, column_selecter: F) -> Self {
        let field = column_selecter(UserFields {
            id: UserField::id,
            name: UserField::name,
            password: UserField::password,
            created_at: UserField::created_at,
            updated_at: UserField::updated_at,
        });
        self.order = Some(aoba::Order::Asc(field.as_str()));
        self
    }
    pub fn ORDER_DESC<F: Fn(UserFields) -> UserField>(mut self, column_selecter: F) -> Self {
        let field = column_selecter(UserFields {
            id: UserField::id,
            name: UserField::name,
            password: UserField::password,
            created_at: UserField::created_at,
            updated_at: UserField::updated_at,
        });
        self.order = Some(aoba::Order::Desc(field.as_str()));
        self
    }
}
struct UserCondition(String);
impl UserCondition {
    fn new() -> Self {
        Self(String::from("WHERE"))
    }
    fn is_empty(&self) -> bool {
        self.0.len() == 5
    }
    fn and(&mut self, new_condition: String) -> Self {
        if self.is_empty() {self.0 += " AND"}
        self.0 += &new_condition;
        self
    }
}
impl UserCondition {
    pub fn id_eq(mut self, another: u32) -> Self {
        self.and(format!(" id = {another}"))
    }
    pub fn id_between(mut self, left: u32, right: u32) -> Self {
        self.and(format!(" id BETWEEN {left} AND {right}"))
    }

    pub fn name_eq<Str: aoba::Str>(mut self, another: Str) -> Self {
        self.and(format!(" name = {}", another.as_str()))
    }
    pub fn name_like<Str: aoba::Str>(mut self, regex: Str) -> Self {
        self.and(format!(" name LIKE {}", regex.as_str()))
    }

    pub fn password_eq<Str: aoba::Str>(mut self, another: Str) -> Self {
        self.and(format!(" password = {}", another.as_str()))
    }
    pub fn password_like<Str: aoba::Str>(mut self, regex: Str) -> Self {
        self.and(format!(" password LIKE {}", regex.as_str()))
    }

    pub fn created_at_eq<D: aoba::AsDateTime>(mut self, another: D) -> Self {
        self.and(format!(" cerated_at = {}", another.as_str()))
    }
    // ...
}

pub(crate) struct Task {
    id: Option<u32>,
    user_id: Option<u32>,
    title: Option<String>,
    body: Option<String>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}
pub(crate) struct CreateTask {
    user_id: u32,
    title: String,
    body: String,
}

// ==========
/*
    $ aoba migrate (= `aoba generate && aoba migrate`)
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
