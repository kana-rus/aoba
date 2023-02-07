use super::schema::{users, User};

fn _user() {
    users::ALL()
        .LIMIT(100)
        .ORDER_ASC(|user| user.name)
        .WHERE(|user| user
            .name_like("%user")
            .id_between(1, 1000)
        );

    users::CREATE(User {
        name: "user1",
        password: "password",
    });

    users::UPDATE()
        .SET(|user| user
            .name("new user")
            .password("new password")
        )
        .WHERE(|user| user
            .id_eq(314)
        );
}
