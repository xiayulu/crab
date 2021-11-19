use crate::user::models::{Account, Profile};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{
    error, get, middleware, web, App, Error, HttpResponse, HttpServer, Responder, Result,
};
use sqlx::postgres::PgPool;
use std::env;

#[get("/users")]
async fn find_all() -> impl Responder {
    todo!()
}

#[get("/users/{id}")]
async fn find() -> impl Responder {
    todo!()
    
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[put("/users/{id}")]
async fn update(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}

pub async fn create_user_view(form: web::Json<Profile>) -> Result<String> {
    Ok(format!("Welcome {:?}!", form.nickname))
}

pub async fn list_user_view() -> Result<HttpResponse, Error> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = PgPool::connect(&db_url)
        .await
        .expect("Error when connect db");

    let recs = sqlx::query!(r#"SELECT * FROM users ORDER BY user_id"#)
        .fetch_all(&db)
        .await
        .expect("Error when get all");

    println!("{:?}", recs);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("list user{:?}", recs)))
}
