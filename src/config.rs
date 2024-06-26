use std::fs;

use serde::Deserialize;

pub const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub address: String,
    pub htmx: Htmx,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Htmx {
    pub source: String,
    pub sha: String,
}

pub fn init() -> Config {
    let cfg: Config = toml::from_str(fs::read_to_string(CONFIG_FILE).unwrap().as_ref()).unwrap();
    cfg
}
