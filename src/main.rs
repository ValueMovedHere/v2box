use anyhow::Result;
use base64::prelude::*;
use clap::Parser;
use ureq;

mod cli;
mod config;
mod parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let encoded_conf = ureq::get(args.subscription_url)
        .header("User-Agent", "curl/8.18.0")
        .call()?
        .body_mut()
        .read_to_string()?;
    let decoded_conf = BASE64_STANDARD.decode(&encoded_conf)?;
    Ok(())
}
