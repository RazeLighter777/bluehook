use std::process::Command;

use rocket::post;

pub fn is_firewall_enabled() -> bool {
    match os_info::get().os_type() {
        os_info::Type::Ubuntu => {
            String::from_utf8(Command::new("ufw").arg("status").output().unwrap().stdout)
                .unwrap()
                .contains("Status: active")
        }
        os_info::Type::CentOS | os_info::Type::RedHatEnterprise => String::from_utf8(
            Command::new("firewall-cmd")
                .arg("--state")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .contains("running"),
        _ => false,
    }
}

#[post("/firewallstatus")]
pub fn firewall_status() -> String {
    format!("Firewall status is {}", is_firewall_enabled())
}
