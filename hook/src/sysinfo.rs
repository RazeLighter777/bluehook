use std::fs::OpenOptions;
use std::io::Write;

use rocket::post;
use serde::Deserialize;

use std::fs;

use crate::auth::Auth;
#[cfg(target_family = "unix")]
fn get_users() -> Vec<String> {
    let contents =
        fs::read_to_string("/etc/passwd").expect("Should have been able to read the file");
    let sc = contents.split("\n");
    let mut res = Vec::new();
    for l in sc {
        let user = l
            .split(":")
            .next()
            .expect("Invalid formatting in passwd")
            .to_owned();
        res.push(user);
    }
    res
}

#[post("/users")]
pub fn user_list_route(auth : Auth) -> String {
    let mut res = String::new();
    for user in get_users() {
        res = res + &user + "\n";
    }
    res
}
