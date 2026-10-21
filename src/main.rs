use anyhow::Result;
use base64::prelude::*;
use clap::Parser;
use serde_json;
use ureq;

use config::Config;

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
    let decoded_conf = BASE64_STANDARD
        .decode(&encoded_conf)
        .expect("Failed to decode");
    let resp_conf_string =
        String::from_utf8(decoded_conf).expect("Failed to create String from decoded response");
    // ---------------------------------
    println!("{resp_conf_string}");
    // -------------------------------------
    let conf_lines = parser::parse_decoded(&resp_conf_string).unwrap();
    let first_conf: Config = serde_json::from_str(conf_lines[1]).unwrap();
    println!("{:#?}", first_conf);

    // let conf_data: Vec<Config> = serde_json::from_str(&resp_conf_string)
    //     .expect("Failed to parse str into configuration data");
    // println!("{:#?}", conf_data);
    Ok(())
}
