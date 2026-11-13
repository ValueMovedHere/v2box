use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, required = true, value_name = "DESTINATION_PATH")]
    pub dest_dir: PathBuf,
    #[arg(short, long, value_name = "SUBSCRIPTION_URL")]
    pub subscription_url: Option<String>,
    #[arg(short, long, value_name = "PROXY")]
    pub proxy: Option<String>,
    #[arg(long, value_name = "PORT", default_value_t = 8080u16)]
    pub port: u16,
    #[arg(short, long, value_name = "BASE64_DECODE", default_value_t = true)]
    pub b64_decode: bool,
}
