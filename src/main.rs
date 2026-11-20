use std::{fs::File, io::Write};

use anyhow::{Result, bail};
use clap::Parser;

use crate::{
    config::{SingBoxConfig, fetch_conf, write_conf_files},
    parser::VlessUri,
};

mod cli;
mod config;
mod errors;
mod parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let raw = args.proxy;
    let dest_dir = args.dest_dir;
    let port = args.port;
    let mut subcription_url = String::new();
    if let Some(url) = args.subscription_url {
        subcription_url = url;
    } else {
        if raw.is_none() {
            bail!("No configuration provided");
        }
        let vless_uri: VlessUri = raw.unwrap().parse().expect("Failed to parse URI");
        let conf = vless_uri
            .to_sing_box_config("proxy", "127.0.0.1", port)
            .unwrap();
        let conf_string = serde_json::to_string(&conf).unwrap();
        let mut dest_file =
            File::create(dest_dir.join("conf.json")).expect("Failed to create configuration file");
        dest_file
            .write_all(conf_string.as_bytes())
            .expect("Failed to write to configuration file");
    }
    let lines = fetch_conf(&subcription_url, args.b64_decode);
    let vless_uris: Vec<VlessUri> = lines
        .into_iter()
        .map(|item| item.parse().unwrap())
        .collect();
    let confs: Vec<SingBoxConfig> = vless_uris
        .into_iter()
        .map(|uri| uri.to_sing_box_config("proxy", "127.0.0.1", port).unwrap())
        .collect();
    write_conf_files(&dest_dir, confs).expect("Failed to write conf files");
    Ok(())
}
