use crate::entity::prelude::*;
use crate::entity::users;
use sea_orm::{prelude::*, ActiveValue, Database, Set, Unset};
use std::env;
use tera::Tera;
use tide::prelude::*;
use tide::{Request, Response};
use tide_tera::prelude::*;

pub async fn create_user_view(req: Request<Tera>) -> tide::Result {
    let tera = req.state();
    let title = "你好";
    // Set value
    let _: ActiveValue<i32> = Set(10);

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    // Unset value
    let _: ActiveValue<i32> = Unset(None);
    let user = users::ActiveModel {
        nickname: Set("sea-orm".to_owned()),
        ..Default::default()
    };

    let res: users::ActiveModel = user.insert(&conn).await?;
    tera.render_response("home.html", &context! { "title" => title })
}
