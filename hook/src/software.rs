use std::process::Command;

pub fn install_package(package_name: &str) {
    println!("Installing {}", package_name);
    match os_info::get().os_type() {
        os_info::Type::Debian | os_info::Type::Ubuntu => {
            Command::new("apt")
                .args(["install", "-y", package_name])
                .output();
        }
        os_info::Type::CentOS | os_info::Type::RedHatEnterprise => {
            Command::new("yum")
                .args(["install", "-y", package_name])
                .output();
        }
        _ => {}
    }
}

pub fn install_recommended_software() {
    println!("Installing recommended software . . . ");
    for pkgname in match os_info::get().os_type() {
        os_info::Type::Debian | os_info::Type::Ubuntu => {
            vec!["screen", "htop", "vim", "nano", "ufw"]
        }
        os_info::Type::CentOS | os_info::Type::RedHatEnterprise => {
            vec!["epel-release", "screen", "vim", "nano", "htop"]
        }
        _ => {
            vec![]
        }
    } {
        install_package(pkgname);
    }
}
