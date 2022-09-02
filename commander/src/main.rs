use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(|| async { "Hello World!" })))
        .bind(("0.0.0.0", 10996))?
        .run()
        .await
}
