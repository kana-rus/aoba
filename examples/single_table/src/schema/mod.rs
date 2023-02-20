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

pub mod model { #![allow(unused, non_snake_case, non_camel_case_types)]
    use super::__private::user::*;

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
        pub fn FIRST(&self) -> GetUser {
            GetUser { condition: aoba::condition::Condition::new() }
        }
    }
}

pub mod __private {
    pub mod user { #![allow(unused, non_snake_case, non_camel_case_types)]
        pub struct ColumnCondition {
            pub id:       aoba::condition::NumberCondition<"id">,
            pub name:     aoba::condition::StringCondition<"name">,
            pub password: aoba::condition::StringCondition<"password">,
        } impl ColumnCondition {
            fn new() -> Self {
                Self {
                    id:       aoba::condition::NumberCondition::new(),
                    name:     aoba::condition::StringCondition::new(),
                    password: aoba::condition::StringCondition::new(),
                }
            }
        }

        pub struct GetUser {
            pub(in super::super) condition: aoba::condition::Condition,
        } impl GetUser {
            #[inline] pub fn WHERE<const N: usize, F: Fn(ColumnCondition)->[aoba::condition::Condition; N]>(mut self, conditions: F) -> Self {
                self.condition = conditions(ColumnCondition::new()).into();
                self
            }
        }
    }
}

