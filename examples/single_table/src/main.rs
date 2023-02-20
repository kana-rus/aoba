mod schema;
use schema::{User, newUser};

async fn _sample_() -> sqlx::Result<()> {
    let dummy_pool = sqlx::postgres::PgPoolOptions::new().connect("").await?;

    {
        let _user = User::FIRST()
            .WHERE(|user| [
                user.id.lt(1000) | user.id.ge(10000),
                user.password.unlike("password"),
            ])
            .save(&dummy_pool).await?;

        let _user = User::FIRST()
            .WHERE(|u| u.name.like("%user"))
            .save(&dummy_pool).await?;
    }

    {
        let _users = User::ALL()
            .WHERE(|u| u.password.eq("password"))
            .ORDER_ASC(|u| u.name)
            .LIMIT(100)
            .save(&dummy_pool).await?;
    }

    {
        let _new_user = User::CREATE(newUser {
            name: "newuser".to_owned(),
            password: "password".to_owned()
        }).save(&dummy_pool).await?;

        let _: () = User::CREATE(newUser {
            name: "newuser".to_owned(),
            password: "password".to_owned()
        }).exec(&dummy_pool).await?;
    }

    {
        let _updated_users = User::UPDATE()
            .SET(|user| user
                .name("new user")
                .password("new_password")
            )
            .LIMIT(10000)
            .save(&dummy_pool).await?;

        let _: () = User::UPDATE()
            .SET(|user| user
                .name("new user")
                .password("new_password")
            )
            .LIMIT(10000)
            .exec(&dummy_pool).await?;
    }

    {
        let _deleted_users = User::DELETE()
            .WHERE(|u| u.id.gt(100000))
            .save(&dummy_pool).await?;

        let _: () = User::DELETE()
            .WHERE(|u| u.password.eq("password") & u.name.eq("user"))
            .exec(&dummy_pool).await?;
    }

    Ok(())
}

fn main() {}
