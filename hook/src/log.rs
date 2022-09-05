use bluelib::{get_platform, Platform};
use tokio::{io, sync::mpsc::Sender};

use linemux::MuxedLines;

use crate::alerts::{Alert, AlertSeverity, AlertType};
use regex::Regex;

pub enum LogFileType {
    Fail2Ban,
    Authlog,
    Syslog,
    Apache,
    Nginx,
}

pub fn setup_log_monitors(tx: Sender<Alert>) {
    //authlog alert matcher
    if get_platform() == Platform::LINUX {
        tokio::spawn(monitor_logfile(
            match os_info::get().os_type() {
                os_info::Type::CentOS | os_info::Type::Redhat | os_info::Type::RedHatEnterprise => {
                    "/var/log/secure"
                }
                _ => "/var/log/auth.log",
            },
            authlog_alert_matcher,
            tx,
        ));
    }
}

pub fn authlog_alert_matcher(log: &str) -> Option<Alert> {
    let authfailure = Regex::new("authentication failure.+ruser=([\\w]*).+user=([\\w]*)").unwrap();
    let sshunknownuser =
        Regex::new("Failed password for invalid user ([\\w]+) from ([^ ]+)").unwrap();
    let sshfailure = Regex::new("Failed password for ([\\w]+) from ([^ ]+).+ssh").unwrap();
    let passwordchanged = Regex::new("password changed for ([\\w]+)").unwrap();
    if let Some(captures) = authfailure.captures(log) {
        Some(Alert(
            AlertType::FailedLogin {
                ruser: if let Some(m) = captures.get(1) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
                user: if let Some(m) = captures.get(2) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
            },
            AlertSeverity::PotentialAttack,
        ))
    } else if let Some(captures) = sshunknownuser.captures(log) {
        Some(Alert(
            AlertType::SshUnknownUser {
                user: if let Some(m) = captures.get(1) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
                host: if let Some(m) = captures.get(2) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
            },
            AlertSeverity::PotentialAttack,
        ))
    } else if let Some(captures) = sshfailure.captures(log) {
        Some(Alert(
            AlertType::SshFailedPassword {
                user: if let Some(m) = captures.get(1) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
                host: if let Some(m) = captures.get(2) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
            },
            AlertSeverity::PotentialAttack,
        ))
    } else if let Some(captures) = passwordchanged.captures(log) {
        Some(Alert(
            AlertType::PasswordChanged {
                user: if let Some(m) = captures.get(1) {
                    m.as_str().to_owned()
                } else {
                    "".to_owned()
                },
            },
            AlertSeverity::PotentialAttack,
        ))
    } else {
        None
    }
}

pub async fn get_next_line(filename: &str) -> io::Result<String> {
    let mut lines = MuxedLines::new()?;
    lines.add_file(filename).await?;
    let res = lines.next_line().await?;
    match res {
        Some(line) => Ok(line.line().to_owned()),
        None => Ok("".to_owned()),
    }
}

pub async fn monitor_logfile(
    filename: &str,
    tp: fn(&str) -> Option<Alert>,
    sender: tokio::sync::mpsc::Sender<Alert>,
) -> io::Result<String> {
    loop {
        let ln = get_next_line(&filename).await?;
        match tp(&ln) {
            Some(alert) => match sender.send(alert).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Logfile {} channel closed due to error: {}", filename, e);
                }
            },
            None => {
                //new line wasn't significant
                println!("New line {} not significant", &ln);
            }
        }
    }
}
