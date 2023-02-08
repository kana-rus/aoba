use super::schema::{
    table::users,
    entity::NewUser,
};

fn _user_() {
    users::ALL()
        .LIMIT(100)
        .ORDER_BY(|u| u.name)
        .ORDER_BY_REVERSED(|u| u.password)
        .WHERE(|u| u
            .name_like("%user")
            .id_between(1, 1000)
        );

    users::FIRST();

    users::CREATE(NewUser {
        name: String::from("user1"),
        password: String::from("password"),
    });

    users::UPDATE()
        .SET(|u| u
            .name("new user")
            .password("new password")
        )
        .WHERE(|u| u
            .id_eq(314)
        );
}
