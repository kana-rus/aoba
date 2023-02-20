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

pub struct users; impl users {
    #[inline] pub fn FIRST() -> __private::GetUser {
        __private::GetUser { condition: aoba::condition::Condition::new() }
    }
}
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

mod __private {
    use super::*;

    pub struct UserCondition {
        pub id:       aoba::condition::NumberCondition<"id">,
        pub name:     aoba::condition::StringCondition<"name">,
        pub password: aoba::condition::StringCondition<"password">,
    }
    impl UserCondition {
        #[inline] fn new() -> Self {
            Self {
                id:       aoba::condition::NumberCondition::new(),
                name:     aoba::condition::StringCondition::new(),
                password: aoba::condition::StringCondition::new(),
            }
        }
    }

    pub struct GetUser {
        pub(in super) condition: aoba::condition::Condition,
    }
    impl GetUser {
        #[inline] pub fn WHERE<const N: usize, F: Fn(UserCondition)->[aoba::condition::Condition; N]>(mut self, conditions: F) -> Self {
            self.condition = conditions(UserCondition::new()).into();
            self
        }
    }
    impl GetUser {
        #[inline] fn build(&self) -> String {
            format!("SELECT id, name, password FROM users WHERE {}", self.condition)
        }
        #[inline] pub async fn save(self, connection_pool: &sqlx::PgPool) -> sqlx::Result<User> {
            sqlx::query_as::<_, User>(&self.build())
                .fetch_one(connection_pool)
                .await
        }
    }
}

