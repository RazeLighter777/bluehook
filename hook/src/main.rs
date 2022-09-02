mod args;
mod reboot;
mod validation;
mod capabilities;
use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use os_info::get;



#[get("/reboot")]
async fn reboot_route() -> impl Responder {
    tokio::spawn(async move  {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        reboot::reboot()
    });
    format!("Rebooting in 3 seconds . . .")
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let info = os_info::get();
    // Print full information:
    println!("OS information: {}", info);
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    if !validation::validate() {
        eprintln!("WARNING : Platform validation failed.");
    }
    let opts = args::Args::parse();
    HttpServer::new(|| App::new()
        .route("/hello", web::get().to(|| async { "Hello World!" }))
        .service(reboot_route))
        .bind((opts.bind_address, opts.bind_port))?
        .run()
        .await
}
