use aoba/* :: schema */;

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

pub mod table { #![allow(unused, non_snake_case, non_camel_case_types)]
    pub struct users;
    impl users {
        pub fn FIRST(&self) -> GetUser {

        }
    }
}

pub mod __private {
    pub mod user { #![allow(unused, non_snake_case, non_camel_case_types)]

        pub mod condition {
            pub struct Condition(String);

            pub struct ColumnCondition {
                pub id: NumberCondition,
                pub name: NameCondition,
                pub password: PasswordCondition,
            }
            pub struct IdCondition {}
            impl Condition {
                pub fn
            }
        }

        pub struct GetUser {
            condition: Condition,
        }
    }
}

