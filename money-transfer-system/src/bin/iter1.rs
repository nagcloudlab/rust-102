// main.rs
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[path = "../iter1/api.rs"]
mod api;
#[path = "../iter1/db.rs"]
mod db;
#[path = "../iter1/model.rs"]
mod model;
#[path = "../iter1/repository.rs"]
mod repository;
#[path = "../iter1/service.rs"]
mod service;

use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let pool = db::init_db().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(api::init_routes)
    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}
