use std::io::Write;
use std::{fs::OpenOptions, io};

use rocket::form::Form;
use rocket::{post, FromForm, Request};
use serde::Deserialize;

use crate::auth::Auth;
#[derive(FromForm)]
pub struct SshFormData {
    pub key: String,
}

pub fn install_ssh_key(key: &str, user: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("/home/{user}/.ssh/authorized_keys"))?;
    return writeln!(file, "{}", key);
}

#[post("/<user>/ssh", data = "<key>")]
pub fn ssh_route(auth: Auth, user: String, key: Form<SshFormData>) -> String {
    "good".to_owned()
}
