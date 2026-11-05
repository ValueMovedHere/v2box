use std::path::PathBuf;

use clap::Parser;
use url::Url;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, required = true, value_name = "DESTINATION_PATH")]
    dest_dir: PathBuf,
    // #[arg(short, long, value_name = "SUBSCRIPTION_URL")]
    // pub subscription_url: String,
    #[arg(short, long, value_name = "PROXY")]
    pub proxy: String,
}
