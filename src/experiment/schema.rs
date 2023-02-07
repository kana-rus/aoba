/*
schema! {
    #[with(id)]
    User {
        name:     CHAR(20),
        password: CHAR(20),
    }
}
*/

use super::aoba;

pub mod users {
    use crate::experiment::aoba;
    use super::{User, __user_private::{UserCreater, UserSelecter, UserUpdater, UserDeleter}};

    #[allow(non_snake_case)]
    #[allow(unused)]
    pub fn CREATE<
        NameStr: aoba::AsStr,
        PasswordStr: aoba::AsStr,
    >(user: User<NameStr, PasswordStr>) -> UserCreater<NameStr, PasswordStr> {
        UserCreater::new(user)
    }
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub fn FIRST() -> UserSelecter {
        UserSelecter::new(Some(1))
    }
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub fn ALL() -> UserSelecter {
        UserSelecter::new(None)
    }
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub fn UPDATE() -> UserUpdater {
        UserUpdater::new()
    }
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub fn DELETE() -> UserDeleter {
        UserDeleter::new()
    }
}

#[allow(unused)]
pub struct User<
    NameStr: aoba::AsStr,
    PasswordStr: aoba::AsStr,
> {
    pub name: NameStr,
    pub password: PasswordStr,
}

pub mod __user_private {
    use crate::experiment::aoba;
    use super::User;

    #[allow(unused)]
    pub struct UpdateUser {
        id: Option<u32>,
        name: Option<String>,
        password: Option<String>,
    }
    impl UpdateUser {
        #[inline]
        pub(super) fn new() -> Self {
            Self { id: None, name: None, password: None }
        }
    }
    impl UpdateUser {
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn name<Str: aoba::AsString>(mut self, name: Str) -> Self {
            self.name = Some(name.as_string());
            self
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn password<Str: aoba::AsString>(mut self, password: Str) -> Self {
            self.password = Some(password.as_string());
            self
        }
    }

    #[allow(unused)]
    pub struct UserFields {
        pub id: UserField,
        pub name: UserField,
        pub password: UserField,
    }

    #[allow(non_camel_case_types)]
    #[allow(unused)]
    pub enum UserField {
        id,
        name,
        password,
    } impl UserField {
        fn as_field_name(self) -> &'static str {
            match self {
                Self::id => "id",
                Self::name => "name",
                Self::password => "password",
            }
        }
    }

    #[allow(unused)]
    pub struct UserCondition(String);
    impl UserCondition {
        #[inline]
        pub(super) fn new() -> Self {
            Self(String::from("WHERE"))
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.0.len() == 5
        }

        #[inline]
        fn and(mut self, new_condition: String) -> Self {
            self.0 += if self.is_empty() {" AND "} else {" "};
            self.0 += &new_condition;
            self
        }
    }
    impl UserCondition {
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn id_eq(mut self, another: u32) -> Self {
            self.and(format!("id = {another}"))
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn id_between(mut self, left: u32, right: u32) -> Self {
            self.and(format!("id BETWEEN {left} AND {right}"))
        }

        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn name_eq<Str: aoba::AsStr>(mut self, another: Str) -> Self {
            self.and(format!("name = '{}'", another.as_str()))
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn name_like<Str: aoba::AsStr>(mut self, regex: Str) -> Self {
            self.and(format!("name LIKE '{}'", regex.as_str()))
        }

        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn password_eq<Str: aoba::AsStr>(mut self, another: Str) -> Self {
            self.and(format!("password = '{}'", another.as_str()))
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn password_like<Str: aoba::AsStr>(mut self, regex: Str) -> Self {
            self.and(format!("password LIKE '{}'", regex.as_str()))
        }
    }

    #[allow(unused)]
    pub struct UserCreater<
        NameStr: aoba::AsStr,
        PasswordStr: aoba::AsStr,
    >(User<NameStr, PasswordStr>);
    impl<
        NameStr: aoba::AsStr,
        PasswordStr: aoba::AsStr,
    > UserCreater<NameStr, PasswordStr> {
        pub(super) fn new(create_user: User<NameStr, PasswordStr>) -> Self {
            Self(create_user)
        }

        // pub async fn exec<'e, E: sqlx::Executor<'e>>(self, executor: E) -> 
    }

    #[allow(unused)]
    pub struct UserSelecter {
        limit:     Option<usize>,
        order:     Option<aoba::Order>,
        condition: UserCondition,
    }
    impl UserSelecter {
        #[allow(unused)]
        pub(super) fn new(limit: Option<usize>) -> Self {
            Self { limit, order: None, condition: UserCondition::new() }
        }
    }
    impl UserSelecter {
        // ======
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn ORDER_ASC<F: Fn(UserFields) -> UserField>(mut self, field_selecter: F) -> Self {
            let field = field_selecter(UserFields {
                id: UserField::id,
                name: UserField::name,
                password: UserField::password,
            });
            self.order = Some(aoba::Order::Asc(field.as_field_name()));
            self
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn ORDER_DESC<F: Fn(UserFields) -> UserField>(mut self, field_selecter: F) -> Self {
            let field = field_selecter(UserFields {
                id: UserField::id,
                name: UserField::name,
                password: UserField::password,
            });
            self.order = Some(aoba::Order::Desc(field.as_field_name()));
            self
        }
        // =====
    }

    #[allow(unused)]
    pub struct UserUpdater {
        set_values: UpdateUser,
        limit:      Option<usize>,
        condition:  UserCondition,
    }
    impl UserUpdater {
        #[allow(unused)]
        pub(super) fn new() -> Self {
            Self { set_values: UpdateUser::new(), limit: None, condition: UserCondition::new() }
        }
    }
    impl UserUpdater {
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn SET<F: Fn(UpdateUser) -> UpdateUser>(mut self, values_setter: F) -> Self {
            self.set_values = values_setter(self.set_values);
            self
        }
        // ======
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        // #[allow(non_snake_case)]
        // #[allow(unused)]
        // pub fn ORDER_ASC<F: Fn(UserFields) -> UserField>(mut self, field_selecter: F) -> Self {
        //     let field = field_selecter(UserFields {
        //         id: UserField::id,
        //         name: UserField::name,
        //         password: UserField::password,
        //     });
        //     self.order = Some(aoba::Order::Asc(field.as_field_name()));
        //     self
        // }
        // #[allow(non_snake_case)]
        // #[allow(unused)]
        // pub fn ORDER_DESC<F: Fn(UserFields) -> UserField>(mut self, field_selecter: F) -> Self {
        //     let field = field_selecter(UserFields {
        //         id: UserField::id,
        //         name: UserField::name,
        //         password: UserField::password,
        //     });
        //     self.order = Some(aoba::Order::Desc(field.as_field_name()));
        //     self
        // }
        // =====
    }

    #[allow(unused)]
    pub struct UserDeleter {
        limit:     Option<usize>,
        condition: UserCondition,
    }
    impl UserDeleter {
        pub(super) fn new() -> Self {
            Self { limit: None, condition: UserCondition::new() }
        }
    }
    impl UserDeleter {
        // ======
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        // #[allow(non_snake_case)]
        // #[allow(unused)]
        // pub fn ORDER_ASC<F: Fn(UserFields) -> UserField>(mut self, field_selecter: F) -> Self {
        //     let field = field_selecter(UserFields {
        //         id: UserField::id,
        //         name: UserField::name,
        //         password: UserField::password,
        //     });
        //     self.order = Some(aoba::Order::Asc(field.as_field_name()));
        //     self
        // }
        // #[allow(non_snake_case)]
        // #[allow(unused)]
        // pub fn ORDER_DESC<F: Fn(UserFields) -> UserField>(mut self, field_selecter: F) -> Self {
        //     let field = field_selecter(UserFields {
        //         id: UserField::id,
        //         name: UserField::name,
        //         password: UserField::password,
        //     });
        //     self.order = Some(aoba::Order::Desc(field.as_field_name()));
        //     self
        // }
        // =====
    }

}