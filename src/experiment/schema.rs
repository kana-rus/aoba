/*
schema! {
    #[with(id)]
    User {
        name:     CHAR(20),
        password: CHAR(20),
    }
}
*/

pub mod column {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    #![allow(non_upper_case_globals)]

    use super::__private::column;

    pub const id: column::id = column::id;
    pub const name: column::name = column::name;
    pub const password: column::password = column::password;
}

pub mod table {
    #![allow(unused, non_snake_case, non_camel_case_types)]

    use crate::experiment::aoba;
    use super::{__private::*, entity::{User, newUser}};

    pub struct users {
        pub id: UserColumn,
        pub name: UserColumn,
        pub password: UserColumn,
    }
    impl users {
        pub fn CREATE(user: newUser) -> CreateUser {
            CreateUser::new(user)
        }
        pub fn FIRST() -> GetUser {
            GetUser::new()
        }
        pub fn ALL() -> GetUsers {
            GetUsers::new()
        }
        pub fn UPDATE() -> UpdateUsers {
            UpdateUsers::new()
        }
        pub fn DELETE() -> DeleteUsers {
            DeleteUsers::new()
        }
    }
}

pub mod entity {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use crate::experiment::aoba;

    pub struct newUser {
        pub name: String,
        pub password: String,
    }

    #[derive(sqlx::FromRow)]
    pub struct User {
        pub id: i64,
        pub name: String,
        pub password: String,
    }
}

pub mod __private {
    #![allow(unused, non_snake_case, non_camel_case_types)]
    use std::marker::PhantomData;
    use sqlx::{FromRow, Value, Row, Database, Executor};
    use super::{entity::{User, newUser}, table::users};
    use crate::experiment::aoba;

    pub mod column {
        pub trait Column {fn name(&self) -> &'static str;}
        pub trait UserColumn {}
        // pub trait TaskColumn {}
        // ...

        pub struct id; const _: () = {
            impl UserColumn for id {}
        };
        pub struct name; const _: () = {
            impl UserColumn for name {}
        };
        pub struct password; const _: () = {
            impl UserColumn for password {}
        };
    }

    pub mod user {
        #![allow(unused, non_snake_case, non_camel_case_types)]
        use super::{column, column::UserColumn};

        pub struct OrderBy(String);
        impl OrderBy {
            pub fn asc<C: UserColumn>(&mut self, column: C) {

            }
        }
    }

    const USER_COLUMNS: users = users {
        id: UserColumn::id,
        name: UserColumn::name,
        password: UserColumn::password,
    };
    pub enum UserColumn {id, name, password,}

    struct OrderUsersBy {
        id: Option<aoba::Order>,
        name: Option<aoba::Order>,
        password: Option<aoba::Order>,
    }
    impl OrderUsersBy {
        #[inline] pub(super) fn new() -> Self {
            Self { id: None, name: None, password: None }
        }
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

    pub struct CreateUser(newUser);
    impl CreateUser {
        #[inline] pub(super) fn new(create_user: newUser) -> Self {
            Self(create_user)
        }
    }
    const _: (/* CreateUser Query Builder */) = {
        impl<'q, DB: sqlx::Database> sqlx::Execute<'q, DB> for CreateUser {
            #[inline] fn statement(&self) -> Option<&<DB as sqlx::database::HasStatement<'q>>::Statement> {None}
            #[inline] fn take_arguments(&mut self) -> Option<<DB as sqlx::database::HasArguments<'q>>::Arguments> {None}
            #[inline] fn persistent(&self) -> bool {true}
            #[inline] fn sql(&self) -> &'q str {
                todo!()
            }
        }
        impl CreateUser {
            #[inline] pub async fn exec<'e, E: sqlx::Executor<'e>>(self, executer: E) -> sqlx::Result<<<E as sqlx::Executor<'e>>::Database as sqlx::Database>::QueryResult> {
                executer
                    .execute(self)
                    .await
            }
            pub async fn save<'e, E: sqlx::Executor<'e>>(self, executer: E) -> sqlx::Result<User>
            where
                &'e str: sqlx::ColumnIndex<<<E as sqlx::Executor<'e>>::Database as sqlx::Database>::Row>,
                i64: sqlx::decode::Decode<'e, <<<E as sqlx::Executor<'e>>::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
                i64: sqlx::types::Type<<<<E as sqlx::Executor<'e>>::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
                String: sqlx::decode::Decode<'e, <<<E as sqlx::Executor<'e>>::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
                String: sqlx::types::Type<<<<E as sqlx::Executor<'e>>::Database as sqlx::Database>::Row as sqlx::Row>::Database>,
            {
                struct Query<'q, DB: Database> {
                    statement: &'q str,
                    database:  PhantomData<DB>,
                }
                struct QueryAs<'q, DB: Database, O> {
                    inner:  Query<'q, DB>,
                    output: PhantomData<O>,
                }

                impl<'q, DB: Database> sqlx::Execute<'q, DB> for Query<'q, DB> {
                    #[inline] fn statement(&self) -> Option<&<DB as sqlx::database::HasStatement<'q>>::Statement> {None}
                    #[inline] fn take_arguments(&mut self) -> Option<<DB as sqlx::database::HasArguments<'q>>::Arguments> {None}
                    #[inline] fn persistent(&self) -> bool {true}
                    #[inline] fn sql(&self) -> &'q str {
                        todo!()
                    }
                }

                impl<'q, DB, O> QueryAs<'q, DB, O>
                where
                    DB: Database,
                    O: Send + Unpin + for<'r> FromRow<'r, DB::Row>,
                {
                    async fn save<'e, 'c: 'e, E>(self, executer: E) -> sqlx::Result<Option<O>>
                    where
                        'q: 'e,
                        E: 'e + Executor<'c, Database = DB>,
                        DB: 'e,
                        O: 'e,
                    {
                        let row = executer.fetch_optional(self.inner).await?;
                        if let Some(row) = row {
                            O::from_row(&row).map(Some)
                        } else {
                            Ok(None)
                        }
                    }
                }

                todo!()
            }
        }
    };            
    pub struct UpdateUsers {
        set_values: ValueSetter,
        limit:      Option<usize>,
        condition:  UserCondition,
    }
    pub struct ValueSetter {
        id: Option<u32>,
        name: Option<String>,
        password: Option<String>,
    }
    impl UpdateUsers {
        #[inline] pub(super) fn new() -> Self {
            Self { set_values: ValueSetter::new(), limit: None, condition: UserCondition::new() }
        }
        #[inline] pub fn SET<F: Fn(ValueSetter) -> ValueSetter>(mut self, values_setter: F) -> Self {
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
    impl ValueSetter {
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

    pub struct DeleteUsers {
        limit:     Option<usize>,
        condition: UserCondition,
    }
    impl DeleteUsers {
        #[inline] pub(super) fn new() -> Self {
            Self { limit: None, condition: UserCondition::new() }
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

    pub struct GetUsers {
        // select:    SelectUserColumns,
        limit:     Option<usize>,
        order:     OrderUsersBy,
        condition: UserCondition,
    }
    impl GetUsers {
        pub(super) fn new() -> Self {
            Self {
                // select:    SelectUserColumns::new(),
                limit:     None,
                order:     OrderUsersBy::new(),
                condition: UserCondition::new(),
            }
        }
        // pub fn SELECT<Columns: Into<SelectColumns>, F: Fn(s) -> Columns>(mut self, select_columns: F) -> Self {
        //     self.select = select_columns(_COLUMNS).into();
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
        pub fn ORDER_ASC<F: Fn(users) -> UserColumn>(mut self, select_column: F) -> Self {
            self.order.set(select_column(USER_COLUMNS), aoba::Order::Asc);
            self
        }
        pub fn ORDER_DESC<F: Fn(users) -> UserColumn>(mut self, select_column: F) -> Self {
            self.order.set(select_column(USER_COLUMNS), aoba::Order::Desc);
            self
        }
    }

    pub struct GetUser {
        // select:    SelectUserColumns,
        condition: UserCondition,
    }
    impl GetUser {
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
    impl<'q, DB: sqlx::Database> sqlx::Execute<'q, DB> for GetUser {
        fn statement(&self) -> Option<&<DB as sqlx::database::HasStatement<'q>>::Statement> {None}
        fn take_arguments(&mut self) -> Option<<DB as sqlx::database::HasArguments<'q>>::Arguments> {None}
        fn persistent(&self) -> bool {true}
    
        fn sql(&self) -> &'q str {
            todo!()
        }
    }
}
