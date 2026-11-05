use anyhow::Result;
// use base64::prelude::*;
use clap::Parser;
use serde_json;
// use ureq;

use crate::parser::VlessUri;

mod cli;
mod config;
mod errors;
mod parser;

fn main() -> Result<()> {
    let raw = cli::Args::parse().proxy;
    let uri: VlessUri = raw.parse()?;
    let conf = uri.to_sing_box_config("proxy", "127.0.0.1", 8080u16)?;
    let conf_string = serde_json::to_string(&conf)?;
    println!("{conf_string}");
    Ok(())
}
