use sqlx::ConnectOptions;

use super::schema::{
    table::users,
    entity::{newUser, User},
};

async fn _user_() {
    let mut conn = sqlx::postgres::PgConnectOptions::new()
        .connect()
        .await
        .unwrap();

    users::ALL()
        .LIMIT(100)
        .ORDER_ASC(|u| u.name)
        .ORDER_DESC(|u| u.password)
        .WHERE(|u| u
            .name_like("%user")
            .id_between(1, 1000)
        );

    users::FIRST();

    let create_query = users::CREATE(newUser {
        name: String::from("user1"),
        password: String::from("password"),
    });
    let result = create_query
        .exec(&mut conn)
        .await;

    let create_query = users::CREATE(newUser {
        name: String::from("user1"),
        password: String::from("password"),
    });
    let created = create_query
        .save(&mut conn)
        .await
        .unwrap();
    let id: i64 = created.id;
    let name: String = created.name;

    users::UPDATE()
        .SET(|u| u
            .name("new user")
            .password("new password")
        )
        .WHERE(|u| u
            .id_eq(314)
        );
}
