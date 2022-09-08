pub struct Hole(pub HoleType, pub HoleSeverity);

pub enum HoleType {
    FirewallDisabled,
    FtpAnonymous,
    OutdatedSystem,
    CronEnabled,
    Fail2BanNotInstalled,
}

pub enum HoleSeverity {
    NotIdeal,
    PrettyBad,
    ReallyBad,
}
