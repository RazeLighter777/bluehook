use pwhash;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::{
    fairing::{Fairing, Info, Kind},
    request::{self, FromRequest},
    Data, Request,
};
use std::io::Write;
use std::{fs, hash::Hash, path::Path};

use rocket::http::{ContentType, Method, Status};
use rocket::Response;

pub struct Auth {}
#[derive(Debug, Clone)]
pub struct AuthError;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get("auth") {
            Some(cook) => {
                if let Ok(res) = verify_password(&cook.value()) {
                    if res {
                        request::Outcome::Success(Auth {})
                    } else {
                        request::Outcome::Failure((
                            rocket::http::Status::Unauthorized,
                            AuthError {},
                        ))
                    }
                } else {
                    println!("Did not verify");
                    request::Outcome::Failure((rocket::http::Status::Unauthorized, AuthError {}))
                }
            }
            None => request::Outcome::Failure((rocket::http::Status::Unauthorized, AuthError {})),
        }
    }
}

pub fn random_string() -> String {
    let x: u64 = thread_rng().gen();
    let y: u64 = thread_rng().gen();
    x.to_string() + &y.to_string()
}

pub fn write_password_file() -> std::io::Result<String> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("./.bluehook_password_hash")?;
    let password = random_string();
    let hs = pwhash::bcrypt::hash(&password);
    write!(f, "{}", hs.unwrap())?;
    Ok(password)
}

pub fn does_passwordfile_exist() -> bool {
    Path::new("./.bluehook_password_hash").exists()
}

pub fn verify_password(pass: &str) -> std::io::Result<bool> {
    let s = &fs::read_to_string("./.bluehook_password_hash")?;
    Ok(pwhash::bcrypt::verify(pass, s))
}
