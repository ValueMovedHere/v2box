use anyhow::Result;

pub fn parse_decoded(decoded_conf: &str) -> Result<Vec<&str>> {
    let lines: Vec<&str> = decoded_conf.lines().collect();
    Ok(lines)
}
