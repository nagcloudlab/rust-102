// main.rs
use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod api;
mod db;
mod models;
mod repository;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init_db().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(api::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
