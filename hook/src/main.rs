mod args;
use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use os_info::get;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let info = os_info::get();
    // Print full information:
    println!("OS information: {}", info);
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());

    let opts = args::Args::parse();
    HttpServer::new(|| App::new().route("/hello", web::get().to(|| async { "Hello World!" })))
        .bind((opts.bind_address, opts.bind_port))?
        .run()
        .await
}
