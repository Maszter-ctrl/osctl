use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub packages: Option<Packages>,
    pub services: Option<Services>,
}

#[derive(Debug, Deserialize)]
pub struct Packages {
    pub install: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Services {
    pub enable: Option<Vec<String>>,
}

pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    serde_yaml::from_str(&content).expect("Invalid YAML config")
}
