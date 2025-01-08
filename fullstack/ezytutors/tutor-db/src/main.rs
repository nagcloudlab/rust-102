use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::http::header;

#[path = "./handlers.rs"]
mod handlers;
#[path = "./routes.rs"]
mod routes;

#[path = "./models.rs"]
mod models;

#[path = "./state.rs"]
mod state;
use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .max_age(3600),
            )
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
