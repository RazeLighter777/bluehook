use system_shutdown::{self, ShutdownResult};



//use cross platform library to reboot on linux and windows.
#[cfg(target_os = "linux")]
pub fn reboot() ->ShutdownResult {
    system_shutdown::reboot()
}
#[cfg(target_os = "windows")]
pub fn reboot() ->ShutdownResult {
    system_shutdown::reboot()
}
//use nix library to reboot on freebsd
#[cfg(target_os = "freebsd")]
pub fn reboot() ->ShutdownResult {
    nix::sys::reboot::reboot(nix::sys::reboot::RebootMode::RB_AUTOBOOT).unwrap();
    return ShutdownResult::Ok(())
}