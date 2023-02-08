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
        pub fn FIRST() -> UserSelecter {
            UserSelecter::new()
        }
        pub fn ALL() -> UsersSelecter {
            UsersSelecter::new()
        }
        pub fn UPDATE() -> UserUpdater {
            UserUpdater::new()
        }
        pub fn DELETE() -> UserDeleter {
            UserDeleter::new()
        }
    }
}

pub mod entity {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use crate::experiment::aoba;

    pub struct NewUser {
        pub name: String,
        pub password: String,
    }

    #[derive(sqlx::FromRow)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub password: String,
    }
}

// mod row {
//     #![allow(unused, non_snake_case, non_camel_case_types)]
//     use crate::experiment::aoba;
// 
//     pub struct User_id_name_password {pub id: u32, pub name: String, pub password: String}
//     pub struct User_id_name {pub id: u32, pub name: String}
//     pub struct User_id_password {pub id: u32, pub password: String}
//     pub struct User_name_password {pub name: String, pub password: String}
//     pub struct User_id {pub id: u32}
//     pub struct User_name {pub name: String}
//     pub struct User_password {pub password: String}
// }

pub mod __private {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use sqlx::{FromRow, Value, Row};

    use super::{entity::User, table::{users}};

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
    // const _: () = {
    //     impl From<users> for SelectUserColumns {
    //         #[inline] fn from(value: users) -> Self {
    //             SelectUserColumns { id: true, name: true, password: true }
    //         }
    //     }
    //     impl From<[UserColumn; 1]> for SelectUserColumns {
    //         #[inline] fn from(value: [UserColumn; 1]) -> Self {
    //             let mut select = Self { id: false, name: false, password: false };
    //             match value[0] {
    //                 UserColumn::id => select.id = true,
    //                 UserColumn::name => select.name = true,
    //                 UserColumn::password => select.password = true,
    //             }
    //             select
    //         }
    //     }
    //     impl From<[UserColumn; 2]> for SelectUserColumns {
    //         #[inline] fn from(value: [UserColumn; 2]) -> Self {
    //             let mut select = Self { id: false, name: false, password: false };
    //             for column in value.into_iter() {
    //                 match column {
    //                     UserColumn::id => select.id = true,
    //                     UserColumn::name => select.name = true,
    //                     UserColumn::password => select.password = true,
    //                 }
    //             }
    //             select
    //         }
    //     }
    //     impl From<[UserColumn; 3]> for SelectUserColumns {
    //         #[inline] fn from(value: [UserColumn; 3]) -> Self {
    //             let mut select = Self { id: false, name: false, password: false };
    //             for column in value.into_iter() {
    //                 match column {
    //                     UserColumn::id => select.id = true,
    //                     UserColumn::name => select.name = true,
    //                     UserColumn::password => select.password = true,
    //                 }
    //             }
    //             select
    //         }
    //     }
    // };
    const USER_COLUMNS: users = users {
        id: UserColumn::id,
        name: UserColumn::name,
        password: UserColumn::password,
    };
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
    impl<'e, 'r, E: sqlx::Executor<'e>> aoba::Query<'e, 'r, E> for UserCreater
    where
        &'r std::primitive::str: sqlx::ColumnIndex<<E::Database as sqlx::Database>::Row>,
        u32: sqlx::decode::Decode<'r, <<E::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
        u32: sqlx::types::Type<<<E::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
        String: sqlx::decode::Decode<'r, <<E::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
        String: sqlx::types::Type<<<E::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
    {
        type Return = User;

        #[inline] async fn exec(self, executer: E) -> Result<aoba::QuerySucceed, sqlx::Error> {
            executer.execute(self).await.map(|_| aoba::QuerySucceed)
        }
        #[inline] async fn save(self, executer: E) -> Result<<Self as aoba::Query<'e, 'r, E>>::Return, sqlx::Error> {
            let row = executer
                .fetch_one(self)
                .await?;
            <User as FromRow<<E::Database as sqlx::Database>::Row>>::from_row(&row)
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
        #[inline] pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
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
        #[inline] pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        #[inline] pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
    }

    pub struct UsersSelecter {
        // select:    SelectUserColumns,
        limit:     Option<usize>,
        order:     OrderUserBy,
        condition: UserCondition,
    }
    impl UsersSelecter {
        pub(super) fn new() -> Self {
            Self {
                // select:    SelectUserColumns::new(),
                limit:     None,
                order:     OrderUserBy::new(),
                condition: UserCondition::new(),
            }
        }
        // pub fn SELECT<Columns: Into<SelectUserColumns>, F: Fn(users) -> Columns>(mut self, select_columns: F) -> Self {
        //     self.select = select_columns(USER_COLUMNS).into();
        //     self
        // }
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        pub fn ORDER_BY<F: Fn(users) -> UserColumn>(mut self, select_column: F) -> Self {
            self.order.set(select_column(USER_COLUMNS), aoba::Order::Asc);
            self
        }
        pub fn ORDER_BY_REVERSED<F: Fn(users) -> UserColumn>(mut self, select_column: F) -> Self {
            self.order.set(select_column(USER_COLUMNS), aoba::Order::Desc);
            self
        }
    }

    pub struct UserSelecter {
        // select:    SelectUserColumns,
        condition: UserCondition,
    }
    impl UserSelecter {
        pub(super) fn new() -> Self {
            Self {
                // select:    SelectUserColumns::new(),
                condition: UserCondition::new(),
            }
        }
        // pub fn SELECT<Columns: Into<SelectUserColumns>, F: Fn(users) -> Columns>(mut self, select_columns: F) -> Self {
        //     self.select = select_columns(USER_COLUMNS).into();
        //     self
        // }
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
    }
    impl<'q, DB: sqlx::Database> sqlx::Execute<'q, DB> for UserSelecter {
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
