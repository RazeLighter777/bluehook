#[derive(Debug)]
pub struct Alert(pub AlertType, pub AlertSeverity);
#[derive(Debug)]
pub enum AlertType {
    Login { user: String },
    AccountCreation { user: String },
    FailedLogin { ruser: String, user: String },
    SshUnknownUser { user: String, host: String },

    SshFailedPassword { user: String, host: String },
    PasswordChanged { user: String },
    Fail2BanLockout,
    CrontabInstalled,
}

#[derive(Debug)]
pub enum AlertSeverity {
    ProbableAttack,
    PotentialAttack,
    Warning,
    Info,
}
