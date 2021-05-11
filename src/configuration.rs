use config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    // Create a default config from the Config module
    let mut settings = config::Config::default();
    // Read config file "configuration", the extension must be json or yaml.
    settings.merge(config::File::with_name("configuration"))?;
    // Try to convert settings into ResultType
    settings.try_into()
}