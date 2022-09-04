use std::fs::OpenOptions;
use std::io::Write;

use actix_web::{post, web, Responder};
use serde::Deserialize;


use std::fs;
#[cfg(target_family = "unix")]
fn get_users() -> Vec<String> {
    let contents =
        fs::read_to_string("/etc/passwd").expect("Should have been able to read the file");
    let sc = contents.split("\n");
    let mut res = Vec::new();
    for l in sc {
        let user  = l.split(":")
        .next()
        .expect("Invalid formatting in passwd")
        .to_owned();
        res.push(user);
        
    }
    res
}

#[post("/users")]
async fn user_list_route() -> impl Responder {
    let mut res = String::new();
    for user in get_users() {
        res = res + &user + "\n";
    }
    res
}
