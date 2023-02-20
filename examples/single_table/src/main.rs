mod schema;
use schema::users;

async fn _sample_() -> sqlx::Result<()> {
    let dummy_pool = sqlx::postgres::PgPoolOptions::new().connect("").await?;

    let _user = users::FIRST()
        .WHERE(|user| [
            (user.id.less_than(1000)).OR (user.id.greater_than(10000)),
            (user.password.NOT.equals("password"))
        ])
        .save(&dummy_pool)
        .await?;

    Ok(())
}

fn main() {}
