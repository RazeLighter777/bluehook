use clap::Parser;
use serde_json::json;

use crate::sysinfo::user_list_route;
use crate::{
    fail2ban::fail2ban_route,
    firewall::firewall_status,
    log::{authlog_alert_matcher, monitor_logfile, setup_log_monitors},
    reboot::reboot_route,
    software::install_recommended_software,
    ssh::ssh_route,
};
use base64::{decode, encode};

mod alerts;
mod args;
mod auth;
mod capabilities;
mod fail2ban;
mod firewall;
mod holes;
mod log;
mod reboot;
mod software;
mod ssh;
mod sysinfo;
mod validation;
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    //do_password_thing();
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    setup_log_monitors(tx);
    tokio::spawn(async move {
        loop {
            if let Some(msg) = rx.recv().await {
                eprintln!("ALERT: {:?}", msg);
            };
        }
    });
    //tokio::task::spawn().await;
    let info = os_info::get();
    // Print full information:
    println!("OS information: {}", info);
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    if !validation::validate() {
        eprintln!("WARNING : Platform validation failed");
    }
    let opts = args::Args::parse();
    //print the password
    if !auth::does_passwordfile_exist() {
        let pwd = auth::write_password_file().expect("Could not write the password file");
        let x = json!({
            "password" : pwd,
            "ip" : opts.ip_address,
            "port" : opts.bind_port,

        });
        let secret = base64::encode(x.to_string());
        println!(
            "ENTER THIS ONE TIME CODE IN THE COMMANDER DASHBOARD : {}",
            secret
        );
    }
    if opts.install_recommended_software {
        tokio::spawn(async {
            install_recommended_software();
        });
    }
    rocket::build().mount(
        "/",
        routes![reboot_route, ssh_route, fail2ban_route, user_list_route],
    )
}
