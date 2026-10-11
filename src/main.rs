use anyhow::Result;
use base64::prelude::*;
use ureq;

mod cli;
mod config;
mod parser;

fn main() -> Result<()> {
    let encoded_conf = ureq::get(SUBSCRIPTION_URL)
        .header("User-Agent", "curl/8.18.0")
        .call()?
        .body_mut()
        .read_to_string()?;
}
