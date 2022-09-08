use std::process::Command;

use rocket::post;

use crate::{auth::Auth, software::install_package};

pub fn setup_fail2ban() -> bool {
    let os_info = os_info::get();
    match os_info.os_type() {
        os_info::Type::Debian | os_info::Type::Ubuntu => {
            install_package("fail2ban");
            Command::new("systemctl")
                .args(["enable", "--now", "fail2ban"])
                .output();
            true
        }
        os_info::Type::CentOS | os_info::Type::RedHatEnterprise => {
            install_package("epel-release");
            install_package("fail2ban");
            Command::new("systemctl")
                .args(["enable", "--now", "fail2ban"])
                .output();
            true
        }
        _ => false,
    }
}

#[post("/fail2bansetup")]
pub fn fail2ban_route(auth: Auth) -> String {
    format!("Setup result was {}", setup_fail2ban())
}
