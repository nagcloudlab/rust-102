// main.rs
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[path = "../iter2/api.rs"]
mod api;
#[path = "../iter2/db.rs"]
mod db;
#[path = "../iter2/model.rs"]
mod model;
#[path = "../iter2/repository.rs"]
mod repository;
#[path = "../iter2/service.rs"]
mod service;

use env_logger;
use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up tracing subscriber with environment filter
    tracing_subscriber::fmt()
        .with_env_filter("info") // Dynamically control log levels via RUST_LOG
        .init();

    tracing::info!("Starting Money Transfer System...");

    let pool = db::init_db().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(api::init_routes)
    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}
