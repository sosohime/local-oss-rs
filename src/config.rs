use config::{Config, ConfigError};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub storage_dir: PathBuf,
    pub server: ServerSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(
                config::Environment::with_prefix("OSS_RS")
                    .separator("_")
                    .try_parsing(true)
            )
            .set_default("storage_dir", "./storage")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .build()?;

        config.try_deserialize()
    }
}
