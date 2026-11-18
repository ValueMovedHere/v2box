use anyhow::Result;
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
    // let proxy_uri: VlessUri = raw.parse()?;
    // let conf = uri.to_sing_box_config("proxy", "127.0.0.1", port)?;
    // let mut dest_file = File::create(dest_dir.join("conf.json"))?;
    // let conf_string = serde_json::to_string(&conf)?;
    // dest_file.write_all(conf_string.as_bytes())?;
    //
    // 如果有提供订阅链接, 使用订阅链接来获取配置
    if let Some(url) = args.subscription_url {
        subcription_url = url;
    }
    let lines = fetch_conf(&subcription_url, args.b64_decode);
    let vless_uris: Vec<VlessUri> = lines
        .into_iter()
        .map(|item| item.parse().unwrap())
        .collect();
    let confs: Vec<SingBoxConfig> = vless_uris
        .into_iter()
        .map(|uri| uri.to_sing_box_config("proxy", "localhost", 8080).unwrap())
        .collect();
    write_conf_files(&dest_dir, confs).expect("Failed to write conf files");
    Ok(())
}
