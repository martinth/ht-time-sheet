use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub working_hours: u8,
    pub ignore_worked_hours_below: u8
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("settings.toml"))?;
        s.merge(Environment::with_prefix("HT"))?;

        s.try_into()
    }
}
