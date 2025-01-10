use actix_web::rt::time::sleep;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use bytes::Bytes;
use futures_util::stream::{self};
use std::time::Duration;

async fn sse_handler() -> impl Responder {
    // Create an infinite stream of events
    let event_stream = stream::unfold(0, |count| async move {
        let message = format!("data: Event {}\n\n", count);
        sleep(Duration::from_secs(1)).await; // Use Actix's runtime for sleep
        Some((
            Ok::<Bytes, actix_web::Error>(Bytes::from(message)),
            count + 1,
        ))
    });

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(event_stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/events", web::get().to(sse_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
