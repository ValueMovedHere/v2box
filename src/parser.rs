use std::fs::read;

use anyhow::Result;

use crate::config::Entry;

pub fn parse_decoded(decoded_conf: &str) -> Result<Vec<Entry>> {
    let lines: Vec<&str> = decoded_conf.lines().collect();
}
