use std::fs::read;

use anyhow::Result;

pub fn parse_decoded(decoded_conf: &str) -> Result<Vec> {
    let lines: Vec<&str> = decoded_conf.lines().collect();
}
