use anyhow::Context;
use config::{Config, ConfigError, File};
use gpui_kit::Global;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Margins {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 8,
            left: 8,
            bottom: 8,
            right: 8,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bar {
    pub margins: Margins,
    pub inner_height: u32,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            margins: Margins::default(),
            inner_height: 16,
        }
    }
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

    pub fn try_load() -> anyhow::Result<Self> {
        let res = Self::new();

        let add_suggestion = matches!(
            &res,
            Err(ConfigError::Foreign(foreign_error))
                if matches!(
                    foreign_error.downcast_ref::<std::io::Error>(),
                    Some(io_error)
                        if io_error.kind() == std::io::ErrorKind::NotFound
                            && io_error.get_ref().is_some_and(|custom_error| {
                                let custom_error = custom_error.to_string();
                                custom_error.starts_with("configuration file \"")
                                    && custom_error.ends_with("\" not found")
                            })
                )
        );

        let res = res.context("failed to load settings");

        if add_suggestion && !std::path::Path::new("config.toml").exists() {
            let example_settings = Self::example();
            let example_settings = toml::to_string_pretty(&example_settings)?;

            std::fs::write("config.toml", example_settings)?;
        }

        res
    }

    pub fn example() -> Self {
        Self::default()
    }
}

impl Global for Settings {}
