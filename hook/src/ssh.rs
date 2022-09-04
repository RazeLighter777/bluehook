use std::fs::OpenOptions;
use std::io::Write;

use actix_web::{post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct SshFormData {
    pub key: String,
}

pub fn install_ssh_key(key: &str, user: &str) -> bool {
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("/home/{user}/.ssh/authorized_keys"))
    {
        Ok(mut file) => match writeln!(file, "{}", key) {
            Ok(_) => true,
            Err(_) => false,
        },
        Err(e) => false,
    }
}

#[post("/{user}/ssh")]
async fn ssh_route(path: web::Path<(String,)>, form: web::Form<SshFormData>) -> impl Responder {
    if install_ssh_key(&form.key, &path.0) {
        "good"
    } else {
        "fail"
    }
}
