use std::fs::read;

use anyhow::Result;

use crate::config::SingBoxConfig;

pub fn parse_decoded(decoded_conf: &str) -> Result<Vec<SingBoxConfig>> {
    let lines: Vec<&str> = decoded_conf.lines().collect();
}
