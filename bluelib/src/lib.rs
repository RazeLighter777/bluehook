#[derive(PartialEq, Eq)]
pub enum Platform {
    WINDOWS,
    LINUX,
    FREEBSD,
}

#[cfg(target_os = "windows")]
pub fn get_platform() -> Platform {
    Platform::WINDOWS
}
#[cfg(target_os = "linux")]
pub fn get_platform() -> Platform {
    Platform::LINUX
}

#[cfg(test)]
mod tests {
    use super::*;
}
