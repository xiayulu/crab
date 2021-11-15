mod entity;
mod user;

use tera::Tera;
use tide::prelude::*;
use tide::{Request, Response};
use tide_tera::prelude::*;
use user::views;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let tera = Tera::new("templates/**/*")?;

    let mut app = tide::with_state(tera);
    app.at("/").get(home);
    app.at("/users").post(views::create_user_view);
    app.at("/public").serve_dir("public/")?;
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn home(req: Request<Tera>) -> tide::Result {
    let tera = req.state();
    let title = "你好";
    tera.render_response("home.html", &context! { "title" => title })
}
