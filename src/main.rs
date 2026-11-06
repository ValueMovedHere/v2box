use std::{fs::File, io::Write};

use anyhow::Result;
use clap::Parser;
use serde_json;

use crate::parser::VlessUri;

mod cli;
mod config;
mod errors;
mod parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let raw = args.proxy;
    let dest_dir = args.dest_dir;
    let port = args.port;
    let uri: VlessUri = raw.parse()?;
    let conf = uri.to_sing_box_config("proxy", "127.0.0.1", port)?;
    let mut dest_file = File::create(dest_dir.join("conf.json"))?;
    let conf_string = serde_json::to_string(&conf)?;
    dest_file.write_all(conf_string.as_bytes())?;
    Ok(())
}
