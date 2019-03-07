use cis_client::settings::CisSettings;
use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct DinoParkSettings {
    pub search_update_endpoint: String,
    pub orgchart_update_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub cis: CisSettings,
    pub dino_park: DinoParkSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let file = env::var("DPL_SETTINGS").unwrap_or_else(|_| String::from(".settings"));
        let mut s = Config::new();
        s.merge(File::with_name(&file))?;
        s.merge(Environment::new().separator("__"))?;
        s.try_into()
    }
}
