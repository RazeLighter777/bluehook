use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub c2c_address: Option<String>,
    #[clap(long, default_value = "0.0.0.0")]
    pub bind_address: String,
    #[clap(short, long, default_value = "10997")]
    pub bind_port: u16,
}
