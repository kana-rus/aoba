use sqlx::ConnectOptions;

use super::aoba::Query;
use super::schema::{
    table::users,
    entity::NewUser,
};

async fn _user_() {
    let mut conn = sqlx::postgres::PgConnectOptions::new()
        .connect()
        .await
        .unwrap();

    users::ALL()
        .LIMIT(100)
        .ORDER_BY(|u| u.name)
        .ORDER_BY_REVERSED(|u| u.password)
        .WHERE(|u| u
            .name_like("%user")
            .id_between(1, 1000)
        );

    users::FIRST();

    let create_query = users::CREATE(NewUser {
        name: String::from("user1"),
        password: String::from("password"),
    });
    let result  = create_query.exec(&mut conn).await;

    let create_query = users::CREATE(NewUser {
        name: String::from("user1"),
        password: String::from("password"),
    });
    let created = create_query.save(&mut conn).await;

    users::UPDATE()
        .SET(|u| u
            .name("new user")
            .password("new password")
        )
        .WHERE(|u| u
            .id_eq(314)
        );
}
