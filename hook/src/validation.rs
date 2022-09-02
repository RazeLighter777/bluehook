#[cfg(target_os = "windows")]
pub fn validate() -> bool {
    true
}
#[cfg(target_os = "linux")]
pub fn validate() -> bool {
    use nix::unistd::getuid;
    if !getuid().is_root() {
        eprintln!("Please run hook as root user");
        false
    } else if os_info::get().os_type() != os_info::Type::Ubuntu || os_info::get().os_type() != os_info::Type::CentOS || os_info::get().os_type() != os_info::Type::Debian ||  os_info::get().os_type() != os_info::Type::Redhat {
        eprintln!("Unsupported OS type {} ", os_info::get().os_type());
        false
    } else {
        true
    }
}