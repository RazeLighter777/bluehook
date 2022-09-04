mod args;
mod capabilities;
mod reboot;
mod ssh;
mod sysinfo;
mod validation;
mod log;

use actix_web::{HttpServer, App, web};
use clap::Parser;

use crate::{reboot::reboot_route, ssh::ssh_route, sysinfo::user_list_route};

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
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello !" }))
            .service(reboot_route)
            .service(ssh_route)
            .service(user_list_route)
    })
    .bind((opts.bind_address, opts.bind_port))?
    .run()
    .await
}
