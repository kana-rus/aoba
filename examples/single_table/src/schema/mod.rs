use aoba::schema;

schema!{
    #[use id, times]
    User {
        name:     VARCHAR(20) where NOT_NULL + DEFAULT("No name"),
        password: VARCHAR(30) where NOT_NULL,
    } where
        CHECK(password != "password")
}
