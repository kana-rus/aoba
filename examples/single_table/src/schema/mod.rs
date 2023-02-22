use aoba::schema;

schema!{
    #[use id, times]
    User {
        name:     VARCHAR(20) where NOT_NULL + DEFAULT(""),
        password: VARCHAR(30) where NOT_NULL + DEFAULT(""),
    }
    where CHECK(password != "password")
}
