use super::schema::{User, CreateUser};

fn _user() {
    let _ = User::ALL()
        .LIMIT(100)
        .ORDER_ASC(|user| user.name)
        .WHERE(|user| user
            .name_like("%user")
            .id_between(1, 1000)
        );

    let _ = User::CREATE(CreateUser {
        name: "user1",
        password: "password",
    });

    let _ = User::UPDATE()
        .SET(|user| user
            .name("new user")
            .password("new password")
        )
        .WHERE(|user| user
            .id_eq(314)
        );
}
