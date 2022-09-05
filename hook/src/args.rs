use clap::Parser;

#[derive(Parser, Debug)]
//Hook is a blueteam monitoring, setup, and command and control script.
#[clap(author, version, about, long_about = None)]
pub struct Args {
    //What IP address is the command and control server on
    #[clap(short, long)]
    pub c2c_address: Option<String>,
    #[clap(long, default_value = "0.0.0.0")]
    pub bind_address: String,
    //Which port the web interface will listen on
    #[clap(short, long, default_value = "10997")]
    pub bind_port: u16,
    //Whether to install a list of recommended software to make the system less annoying
    #[clap(short, long, default_value_t = true)]
    pub install_recommended_software: bool,
}
