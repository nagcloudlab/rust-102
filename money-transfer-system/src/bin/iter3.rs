use std::path;

// main.rs
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[path = "../iter3/api.rs"]
mod api;
#[path = "../iter3/db.rs"]
mod db;
#[path = "../iter3/model.rs"]
mod model;
#[path = "../iter3/repository.rs"]
mod repository;
#[path = "../iter3/service.rs"]
mod service;

use tracing_subscriber;

#[path = "../iter3/kafka.rs"]
mod kafka;
use kafka::KafkaProducer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let kafka_producer = KafkaProducer::new("localhost:9092", "transfer_topic");
    let pool = db::init_db().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(kafka_producer.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(api::init_routes)
    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}
