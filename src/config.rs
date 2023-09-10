use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub urls: UrlsConfig,
}

#[derive(Debug, Deserialize)]
pub struct UrlsConfig {
    pub libre_translate: String,
    pub anki: String,
}

lazy_static! {
    pub static ref CONFIG: AppConfig = {
        let mut file = File::open("config.toml").expect("Config file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read config file");

        toml::from_str(&contents).expect("Invalid TOML in config file")
    };
}