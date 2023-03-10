/*
schema! {
    #[with(id)]
    User {
        name:     CHAR(20),
        password: CHAR(20),
    }
}
*/

pub mod table {
    #![allow(unused, non_snake_case, non_camel_case_types)]

    use crate::experiment::aoba;
    use super::{__private::*, entity::User};

    pub struct users {
        pub id: UserColumn,
        pub name: UserColumn,
        pub password: UserColumn,
    }
    impl users {
        pub fn CREATE(user: User) -> UserCreater {
            UserCreater::new(user)
        }
        pub fn FIRST<const COLUMNS: &'static [UserColumn]>() -> UserSelecter<{select_pattern(COLUMNS)}> {
            UserSelecter::new()
        }
        pub fn ALL<const COLUMNS: &'static [UserColumn]>() -> UsersSelecter<{select_pattern(COLUMNS)}> {
            UsersSelecter::new()
        }
        pub fn UPDATE() -> UserUpdater {
            UserUpdater::new()
        }
        pub fn DELETE() -> UserDeleter {
            UserDeleter::new()
        }
    }

    #[inline] const fn select_pattern(columns: &'static [UserColumn]) -> usize {
        columns.iter().fold(0, |it, next| )
    }
}

pub mod entity {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use crate::experiment::aoba;

    pub struct User {
        pub name: String,
        pub password: String,
    }
}

mod row {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use crate::experiment::aoba;

    pub struct User_id_name_password {pub id: u32, pub name: String, pub password: String}
    pub struct User_id_name {pub id: u32, pub name: String}
    pub struct User_id_password {pub id: u32, pub password: String}
    pub struct User_name_password {pub name: String, pub password: String}
    pub struct User_id {pub id: u32}
    pub struct User_name {pub name: String}
    pub struct User_password {pub password: String}
}

pub mod __private {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use super::{entity::User, table::users};

    use crate::experiment::aoba;

    // pub struct SelectUserColumns {
    //     id: bool,
    //     name: bool,
    //     password: bool,
    // }
    // impl SelectUserColumns {
    //     #[inline] pub(super) fn new() -> Self {
    //         Self { id: true, name: true, password: true }
    //     }
    // }
    // impl From<UserColumns> for SelectUserColumns {
    //     #[inline] fn from(value: UserColumns) -> Self {
    //         SelectUserColumns { id: true, name: true, password: true }
    //     }
    // }
    // impl<const N: usize> From<[UserColumn; N]> for SelectUserColumns {
    //     #[inline] fn from(value: [UserColumn; N]) -> Self {
    //         let mut select = Self { id: false, name: false, password: false };
    //         for column in value.into_iter() {
    //             match column {
    //                 UserColumn::id => select.id = true,
    //                 UserColumn::name => select.name = true,
    //                 UserColumn::password => select.password = true,
    //             }
    //         }
    //         select
    //     }
    // }
    // pub struct UserColumns {
    //     pub id: UserColumn,
    //     pub name: UserColumn,
    //     pub password: UserColumn,
    // }
    // const USER_COLUMNS: UserColumns = UserColumns {
    //     id: UserColumn::id,
    //     name: UserColumn::name,
    //     password: UserColumn::password,
    // };
    // impl Into<[UserColumn; 3]> for UserColumns {
    //     fn into(self) -> [UserColumn; 3] {
    //         [self.id, self.name, self.password]
    //     }
    // }
    const USER_COLUMNS: users = users {
        id: UserColumn::id,
        name: UserColumn::name,
        password: UserColumn::password,
    };
    #[derive(PartialEq, Eq)]
    pub enum UserColumn {
        id,
        name,
        password,
    } impl UserColumn {
        #[inline] fn as_name(self) -> &'static str {
            match self {
                Self::id => "id",
                Self::name => "name",
                Self::password => "password",
            }
        }
    }

    struct OrderUserBy {
        id: Option<aoba::Order>,
        name: Option<aoba::Order>,
        password: Option<aoba::Order>,
    }
    impl OrderUserBy {
        #[inline] pub(super) fn new() -> Self {
            Self { id: None, name: None, password: None }
        }
    }
    impl OrderUserBy {
        #[inline] fn set(&mut self, column: UserColumn, order: aoba::Order) {
            match column {
                UserColumn::id => self.id = Some(order),
                UserColumn::name => self.name = Some(order),
                UserColumn::password => self.password = Some(order),
            }
        }
    }

    pub struct UserCondition(String);
    impl UserCondition {
        #[inline] pub(super) fn new() -> Self {
            Self(String::from("WHERE"))
        }
        #[inline] fn is_empty(&self) -> bool {
            self.0.len() == 5
        }
        #[inline] fn and(mut self, new_condition: String) -> Self {
            self.0 += if self.is_empty() {" AND "} else {" "};
            self.0 += &new_condition;
            self
        }
    }
    impl UserCondition {
        #[inline] pub fn id_eq(mut self, another: u32) -> Self {
            self.and(format!("id = {another}"))
        }
        #[inline] pub fn id_between(mut self, left: u32, right: u32) -> Self {
            self.and(format!("id BETWEEN {left} AND {right}"))
        }

        #[inline] pub fn name_eq<Str: aoba::string>(mut self, another: Str) -> Self {
            self.and(format!("name = '{}'", another.as_str()))
        }
        #[inline] pub fn name_like<Str: aoba::string>(mut self, regex: Str) -> Self {
            self.and(format!("name LIKE '{}'", regex.as_str()))
        }

        #[inline] pub fn password_eq<Str: aoba::string>(mut self, another: Str) -> Self {
            self.and(format!("password = '{}'", another.as_str()))
        }
        #[inline] pub fn password_like<Str: aoba::string>(mut self, regex: Str) -> Self {
            self.and(format!("password LIKE '{}'", regex.as_str()))
        }
    }

    pub struct UserCreater(
        User
    );
    impl UserCreater {
        #[inline] pub(super) fn new(create_user: User) -> Self {
            Self(create_user)
        }

        // #[inline] pub fn RETURNING<F: Fn(UserColumns) -> U, U: Into<SelectUserColumns>>(
        //     self,
        //     select_columns: F,
        // ) -> UserCreaterReturning {
        //     let columns = select_columns(USER_COLUMNS).into();
        //     UserCreaterReturning { entity: self.0, columns }
        // }
    }
    impl<'q, DB: sqlx::Database> sqlx::Execute<'q, DB> for UserCreater {
        fn statement(&self) -> Option<&<DB as sqlx::database::HasStatement<'q>>::Statement> {None}
        fn take_arguments(&mut self) -> Option<<DB as sqlx::database::HasArguments<'q>>::Arguments> {None}
        fn persistent(&self) -> bool {true}
        fn sql(&self) -> &'q str {
            todo!()
        }
    }
    impl<'r, R: sqlx::Row> aoba::Exec<'r, R> for UserCreater {
        type Return = aoba::QuerySucceed;
        #[inline] async fn exec<'e, E: sqlx::Executor<'e>>(self, executer: E) -> Result<<Self as aoba::Exec<'r, R>>::Return, sqlx::Error> {
            executer.execute(self).await.map(|_| aoba::QuerySucceed)
        }
    }

    // pub struct UserCreaterReturning {
    //     entity:  User,
    //     columns: SelectUserColumns,
    // }
    // impl<'q, DB: sqlx::Database> sqlx::Execute<'q, DB> for UserCreaterReturning {
    //     fn statement(&self) -> Option<&<DB as sqlx::database::HasStatement<'q>>::Statement> {None}
    //     fn take_arguments(&mut self) -> Option<<DB as sqlx::database::HasArguments<'q>>::Arguments> {None}
    //     fn persistent(&self) -> bool {true}
    //     fn sql(&self) -> &'q str {
    //         todo!()
    //     }
    // }

    pub struct UserUpdater {
        set_values: UpdateUser,
        limit:      Option<usize>,
        condition:  UserCondition,
    } impl UserUpdater {
        #[inline] pub(super) fn new() -> Self {
            Self { set_values: UpdateUser::new(), limit: None, condition: UserCondition::new() }
        }
        #[inline] pub fn SET<F: Fn(UpdateUser) -> UpdateUser>(mut self, values_setter: F) -> Self {
            self.set_values = values_setter(self.set_values);
            self
        }
        // ======
        #[inline] pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        // =====
    }
    pub struct UpdateUser {
        id: Option<u32>,
        name: Option<String>,
        password: Option<String>,
    } impl UpdateUser {
        #[inline] pub(super) fn new() -> Self {
            Self { id: None, name: None, password: None }
        }
        #[inline] pub fn name<Str: aoba::string>(mut self, name: Str) -> Self {
            self.name = Some(name.to_string());
            self
        }
        #[inline] pub fn password<Str: aoba::string>(mut self, password: Str) -> Self {
            self.password = Some(password.to_string());
            self
        }
    }

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
        #[inline] pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        // =====
    }

    pub struct UsersSelecter<const SELET_PATTERN: usize> {
        limit:     Option<usize>,
        order:     OrderUserBy,
        condition: UserCondition,
    }
    impl<const SELET_PATTERN: usize> UsersSelecter<SELET_PATTERN> {
        pub(super) fn new() -> Self {
            Self {
                limit: None,
                order: OrderUserBy::new(),
                condition: UserCondition::new(),
            }
        }
        // ======
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        pub fn ORDER_BY(mut self, column: UserColumn) -> Self {
            self.order.set(column, aoba::Order::Asc);
            self
        }
        pub fn ORDER_BY_REVERSED(mut self, column: UserColumn) -> Self {
            self.order.set(column, aoba::Order::Desc);
            self
        }
        // =====
    }

    pub struct UserSelecter<const SELET_PATTERN: usize> {
        condition: UserCondition,
    }
    impl<const SELET_PATTERN: usize> UserSelecter<SELET_PATTERN> {
        pub(super) fn new() -> Self {
            Self { condition: UserCondition::new() }
        }
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
    }
    impl<'q, const SELET_PATTERN: usize, DB: sqlx::Database> sqlx::Execute<'q, DB> for UserSelecter<SELET_PATTERN> {
        fn sql(&self) -> &'q str {
            todo!()
        }
        fn statement(&self) -> Option<&<DB as sqlx::database::HasStatement<'q>>::Statement> {
            None
        }
        fn take_arguments(&mut self) -> Option<<DB as sqlx::database::HasArguments<'q>>::Arguments> {
            None
        }
        fn persistent(&self) -> bool {
            true
        }
    }
}
