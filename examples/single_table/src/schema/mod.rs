/* use aoba::schema; */

/*
    schema! {
        #[with(id)]
        User {
            name:     CHAR(20),
            password: CHAR(20),
        }
    }
*/

// expanded:

#![allow(unused, non_snake_case, non_camel_case_types)]

#[derive(sqlx::FromRow)]
pub struct User {
    pub id:       i32,
    pub name:     String,
    pub password: String,
}
pub struct newUser {
    pub name:     String,
    pub password: String,
}
impl User {
    #[inline] pub fn FIRST() -> __private::GetUser {
        __private::GetUser { condition: aoba::condition::Condition::new() }
    }
    #[inline] pub fn ALL() -> __private::GetUsers {
        __private::GetUsers { condition: aoba::condition::Condition::new(), limit: aoba::limit::Limit::new(), order: aoba::order::OrderBy::new() }
    }
    #[inline] pub fn CREATE(new: newUser) -> __private::CreateUser {
        __private::CreateUser(new)
    }
    #[inline] pub fn UPDATE() -> __private::UpdateUsers {
        __private::UpdateUsers { set: __private::SetUser::new(), condition: aoba::condition::Condition::new(), limit: aoba::limit::Limit::new() }
    }
    #[inline] pub fn DELETE() -> __private::DeleteUsers {
        __private::DeleteUsers { condition: aoba::condition::Condition::new(), limit: aoba::limit::Limit::new() }
    }
}

mod __private {
    use super::*;

    pub trait Column {fn name(&self) -> &'static str;}
    pub struct id; impl Column for id {#[inline] fn name(&self) -> &'static str {"id"}}
    pub struct name; impl Column for name {#[inline] fn name(&self) -> &'static str {"name"}}
    pub struct password; impl Column for password {#[inline] fn name(&self) -> &'static str {"password"}}

    const USERS_COLUMNS: UserColumns = UserColumns {id, name, password};
    pub struct UserColumns {
        pub id: id,
        pub name: name,
        pub password: password,
    }

    const USERS_CONDITION: UserCondition = UserCondition {
        id:       aoba::condition::NumberCondition,
        name:     aoba::condition::StringCondition,
        password: aoba::condition::StringCondition,
    };
    pub struct UserCondition {
        pub id:       aoba::condition::NumberCondition<"id">,
        pub name:     aoba::condition::StringCondition<"name">,
        pub password: aoba::condition::StringCondition<"password">,
    }

    pub struct GetUser {
        pub(super) condition: aoba::condition::Condition,
    }
    impl GetUser {
        #[inline] pub fn WHERE<IntoCondition: Into<aoba::condition::Condition>, F: Fn(UserCondition)->IntoCondition>(mut self, conditions: F) -> Self {
            self.condition = conditions(USERS_CONDITION).into();
            self
        }
    }
    impl GetUser {
        #[inline] pub async fn save(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<User> {
            sqlx::query_as(&format!("SELECT * FROM users {}", self.condition))
                .fetch_one(connection_pool)
                .await
        }
    }

    pub struct GetUsers {
        pub(super) condition: aoba::condition::Condition,
        pub(super) limit: aoba::limit::Limit,
        pub(super) order: aoba::order::OrderBy,
    }
    impl GetUsers {
        #[inline] pub fn WHERE<IntoCondition: Into<aoba::condition::Condition>, F: Fn(UserCondition)->IntoCondition>(mut self, conditions: F) -> Self {
            self.condition = conditions(USERS_CONDITION).into();
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit.set(limit);
            self
        }
        #[inline] pub fn ORDER_ASC<C: Column, F: Fn(UserColumns) -> C>(mut self, column: F) -> Self {
            self.order.ASC(column(USERS_COLUMNS).name());
            self
        }
        #[inline] pub fn ORDER_DESC<C: Column, F: Fn(UserColumns) -> C>(mut self, column: F) -> Self {
            self.order.DESC(column(USERS_COLUMNS).name());
            self
        }
    }
    impl GetUsers {
        #[inline] pub async fn save(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<Vec<User>> {
            sqlx::query_as(&format!("SELECT * FROM users {} {} {}", self.condition, self.limit, self.order))
                .fetch_all(connection_pool)
                .await
        }
    }

    pub struct CreateUser(pub(super) newUser);
    impl CreateUser {
        #[inline] pub async fn save(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<User> {
            sqlx::query_as("INSERT INTO users (name, password) VALUES ($1, $2) RETURNING *")
                .bind(self.0.name)
                .bind(self.0.password)
                .fetch_one(connection_pool)
                .await
        }
        #[inline] pub async fn exec(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<()> {
            sqlx::query(&format!("INSERT INTO users (name, password) VALUES ($1, $2)"))
                .bind(self.0.name)
                .bind(self.0.password)
                .execute(connection_pool)
                .await
                .map(|_| ())
        }
    }

    pub struct UpdateUsers {
        pub(super) set: SetUser,
        pub(super) condition: aoba::condition::Condition,
        pub(super) limit: aoba::limit::Limit,
    }
        pub struct SetUser(String);
        impl SetUser {
            #[inline] pub(super) fn new() -> Self {Self(String::new())}
            #[inline] pub fn name<S: aoba::condition::Str>(mut self, new_name: S) -> Self {
                self.0 += &format!(" name = '{}'", new_name.as_str());
                self
            }
            #[inline] pub fn password<S: aoba::condition::Str>(mut self, new_password: S) -> Self {
                self.0 += &format!(" password = '{}'", new_password.as_str()); 
                self
            }
        }
        impl std::fmt::Display for SetUser {
            #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0.is_empty() {write!(f, "")} else {write!(f, "SET{}", self.0)}
            }
        }
    impl UpdateUsers {
        #[inline] pub fn SET<F: Fn(SetUser)->SetUser>(mut self, set: F) -> Self {
            self.set = set(SetUser::new());
            self
        }
        #[inline] pub fn WHERE<IntoCondition: Into<aoba::condition::Condition>, F: Fn(UserCondition)->IntoCondition>(mut self, conditions: F) -> Self {
            self.condition = conditions(USERS_CONDITION).into();
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit.set(limit);
            self
        }
    }
    impl UpdateUsers {
        #[inline] pub async fn save(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<Vec<User>> {
            sqlx::query_as(&format!("UPDATE users {} {} {} RETURNING *", self.set, self.condition, self.limit))
                .fetch_all(connection_pool)
                .await
        }
        #[inline] pub async fn exec(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<()> {
            sqlx::query(&format!("UPDATE users {} {} {}", self.set, self.condition, self.limit))
                .execute(connection_pool)
                .await
                .map(|_| ())
        }
    }

    pub struct DeleteUsers {
        pub(super) condition: aoba::condition::Condition,
        pub(super) limit: aoba::limit::Limit,
    }
    impl DeleteUsers {
        #[inline] pub fn WHERE<IntoCondition: Into<aoba::condition::Condition>, F: Fn(UserCondition)->IntoCondition>(mut self, conditions: F) -> Self {
            self.condition = conditions(USERS_CONDITION).into();
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit.set(limit);
            self
        }
    }
    impl DeleteUsers {
        #[inline] pub async fn save(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<Vec<User>> {
            sqlx::query_as(&format!("DELETE FROM users {} {} RETURNING *", self.condition, self.limit))
                .fetch_all(connection_pool)
                .await
        }
        #[inline] pub async fn exec(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<()> {
            sqlx::query(&format!("DELETE FROM users {} {}", self.condition, self.limit))
                .execute(connection_pool)
                .await
                .map(|_| ())
        }
    }
}

