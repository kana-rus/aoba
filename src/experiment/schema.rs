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

    pub struct users;
    impl users {
        pub fn CREATE<NameStr: Into<aoba::string>, PasswordStr: Into<aoba::string>>(user: User<NameStr, PasswordStr>) -> UserCreater<NameStr, PasswordStr> {
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

    pub struct User<NameStr: Into<aoba::string>, PasswordStr: Into<aoba::string>> {
        pub name: NameStr,
        pub password: PasswordStr,
    }
}

pub mod __private {
    #![allow(unused, non_snake_case, non_camel_case_types)]

    use crate::experiment::aoba;

    use super::entity::User;

    pub struct UpdateUser {
        id: Option<u32>,
        name: Option<aoba::string>,
        password: Option<aoba::string>,
    }
    impl UpdateUser {
        #[inline]
        pub(super) fn new() -> Self {
            Self { id: None, name: None, password: None }
        }
    }
    impl UpdateUser {
        pub fn name<Str: Into<aoba::string>>(mut self, name: Str) -> Self {
            self.name = Some(name.into());
            self
        }
        pub fn password<Str: Into<aoba::string>>(mut self, password: Str) -> Self {
            self.password = Some(password.into());
            self
        }
    }

    pub struct UserColumns {
        pub id: UserColumn,
        pub name: UserColumn,
        pub password: UserColumn,
    }
    const USER_COLUMNS: UserColumns = UserColumns {
        id: UserColumn::id,
        name: UserColumn::name,
        password: UserColumn::password,
    };
    pub enum UserColumn {
        id,
        name,
        password,
    } impl UserColumn {
        fn as_column_name(self) -> &'static str {
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
        pub(super) fn new() -> Self {
            Self { id: None, name: None, password: None }
        }
    }
    impl OrderUserBy {
        fn set(&mut self, column: UserColumn, order: aoba::Order) {
            match column {
                UserColumn::id => self.id = Some(order),
                UserColumn::name => self.name = Some(order),
                UserColumn::password => self.password = Some(order),
            }
        }
    }

    struct SelectUserColumns {
        id: bool,
        name: bool,
        password: bool,
    }
    impl SelectUserColumns {
        pub(super) fn new() -> Self {
            Self { id: true, name: true, password: true }
        }
    }
    impl SelectUserColumns {
        fn from<const N: usize>(columns: [UserColumn; N]) -> Self {
            let mut select = Self { id: false, name: false, password: false };
            for column in columns.into_iter() {
                match column {
                    UserColumn::id => select.id = true,
                    UserColumn::name => select.name = true,
                    UserColumn::password => select.password = true,
                }
            }
            select
        }
    }

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
        pub fn id_eq(mut self, another: u32) -> Self {
            self.and(format!("id = {another}"))
        }
        pub fn id_between(mut self, left: u32, right: u32) -> Self {
            self.and(format!("id BETWEEN {left} AND {right}"))
        }

        pub fn name_eq<Str: Into<aoba::string>>(mut self, another: Str) -> Self {
            self.and(format!("name = '{}'", another.into().as_str()))
        }
        pub fn name_like<Str: Into<aoba::string>>(mut self, regex: Str) -> Self {
            self.and(format!("name LIKE '{}'", regex.into().as_str()))
        }

        pub fn password_eq<Str: Into<aoba::string>>(mut self, another: Str) -> Self {
            self.and(format!("password = '{}'", another.into().as_str()))
        }
        pub fn password_like<Str: Into<aoba::string>>(mut self, regex: Str) -> Self {
            self.and(format!("password LIKE '{}'", regex.into().as_str()))
        }
    }

    pub struct UserCreater<NameStr: Into<aoba::string>, PasswordStr: Into<aoba::string>>(User<NameStr, PasswordStr>);
    impl<NameStr: Into<aoba::string>, PasswordStr: Into<aoba::string>> UserCreater<NameStr, PasswordStr> {
        pub(super) fn new(create_user: User<NameStr, PasswordStr>) -> Self {
            Self(create_user)
        }
    }

    pub struct UserUpdater {
        set_values: UpdateUser,
        limit:      Option<usize>,
        condition:  UserCondition,
    }
    impl UserUpdater {
        pub(super) fn new() -> Self {
            Self { set_values: UpdateUser::new(), limit: None, condition: UserCondition::new() }
        }
    }
    impl UserUpdater {
        pub fn SET<F: Fn(UpdateUser) -> UpdateUser>(mut self, values_setter: F) -> Self {
            self.set_values = values_setter(self.set_values);
            self
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
        // =====
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
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
        pub fn LIMIT(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        // =====
    }

    pub struct UsersSelecter {
        columns:   SelectUserColumns,
        limit:     Option<usize>,
        order:     OrderUserBy,
        condition: UserCondition,
    }
    impl UsersSelecter {
        pub(super) fn new() -> Self {
            Self {
                columns: SelectUserColumns::new(),
                limit: None,
                order: OrderUserBy::new(),
                condition: UserCondition::new(),
            }
        }
    }
    impl UsersSelecter {
        pub fn SELECT<const N: usize, F: Fn(UserColumns) -> [UserColumn; N]>(mut self, columns_selector: F) -> Self {
            self.columns = SelectUserColumns::from(columns_selector(USER_COLUMNS));
            self
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
        pub fn ORDER_BY<F: Fn(UserColumns) -> UserColumn>(mut self, column_selecter: F) -> Self {
            self.order.set(column_selecter(USER_COLUMNS), aoba::Order::Asc);
            self
        }
        pub fn ORDER_BY_REVERSED<F: Fn(UserColumns) -> UserColumn>(mut self, column_selecter: F) -> Self {
            self.order.set(column_selecter(USER_COLUMNS), aoba::Order::Desc);
            self
        }
        // =====
    }

    pub struct UserSelecter {
        columns:   SelectUserColumns,
        condition: UserCondition,
    }
    impl UserSelecter {
        pub(super) fn new() -> Self {
            Self { columns: SelectUserColumns::new(), condition: UserCondition::new() }
        }
    }
    impl UserSelecter {
        pub fn SELECT<const N: usize, F: Fn(UserColumns) -> [UserColumn; N]>(mut self, columns_selector: F) -> Self {
            self.columns = SelectUserColumns::from(columns_selector(USER_COLUMNS));
            self
        }
        pub fn WHERE<F: Fn(UserCondition) -> UserCondition>(mut self, condition_builder: F) -> Self {
            self.condition = condition_builder(self.condition);
            self
        }
    }

    pub trait UserRow<'r, R: sqlx::Row>: sqlx::FromRow<'r, R> {
        fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
            <Self as sqlx::FromRow<'r, R>>::from_row(row)
        }
    }
    // impl UserCreater<>
}