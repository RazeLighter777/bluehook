mod args;
use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;

#[tokio::main] 
async fn main() -> std::io::Result<()> {
    let opts = args::Args::parse();
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
    .bind((opts.bind_address, opts.bind_port))?
    .run()
    .await
}
