mod schema;
use schema::{User, newUser};

async fn _sample_() -> sqlx::Result<()> {
    let p = sqlx::postgres::PgPoolOptions::new().connect("").await?;

    {
        let _user = User::FIRST()
            .WHERE(|user| [
                user.id.lt(1000) | user.id.ge(10000),
                user.password.unlike("%password%"),
            ])
            .save(&p).await?;

        let _user = User::FIRST()
            .WHERE(|u| u.name.like("%user"))
            .save(&p).await?;
    }

    {
        let _users = User::ALL()
            .WHERE(|u| u.password.eq("password"))
            .ORDER_ASC(|u| u.name)
            .LIMIT(100)
            .save(&p).await?;
    }

    {
        let _new_user = User::CREATE(newUser {
            name: "newuser".to_owned(),
            password: "password".to_owned()
        }).save(&p).await?;

        let _: () = User::CREATE(newUser {
            name: "newuser".to_owned(),
            password: "password".to_owned()
        }).exec(&p).await?;
    }

    {
        let _updated_users = User::UPDATE()
            .SET(|user| user
                .name("new user")
                .password("new_password")
            )
            .LIMIT(10000)
            .save(&p).await?;

        let _: () = User::UPDATE()
            .SET(|user| user
                .name("new user")
                .password("new_password")
            )
            .LIMIT(10000)
            .exec(&p).await?;
    }

    {
        let _deleted_users = User::DELETE()
            .WHERE(|u| u.id.gt(100000))
            .save(&p).await?;

        let _: () = User::DELETE()
            .WHERE(|u| u.password.eq("password") & u.name.eq("user"))
            .exec(&p).await?;
    }

    Ok(())
}

fn main() {}
