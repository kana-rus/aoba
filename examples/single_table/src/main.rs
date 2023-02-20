mod schema;
use schema::users;

async fn _sample_() -> sqlx::Result<()> {
    let dummy_pool = sqlx::postgres::PgPoolOptions::new().connect("").await?;

    let _user = users::FIRST()
        .WHERE(|user| [
            user.id.lt(1000) .OR (user.id.ge(10000)),
            user.password.unlike("password"),
        ])
        .save(&dummy_pool)
        .await?;

    let _user = users::FIRST()
        .WHERE(|u| u.name.like("%user"))
        .save(&dummy_pool)
        .await?;

    Ok(())
}

fn main() {}
