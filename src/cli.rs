use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, required = true, value_name = "DESTINATION_PATH")]
    dest_path: PathBuf,
    #[arg(short, long, required = true, value_name = "SUBSCRIPTION_URL")]
    pub subscription_url: String,
}
