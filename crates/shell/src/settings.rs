use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Margins {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Bar {
    pub margins: Margins,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub bar: Bar,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder().add_source(File::with_name("config"));

        settings.build()?.try_deserialize()
    }
}
