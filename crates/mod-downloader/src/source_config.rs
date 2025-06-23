use std::fmt::Debug;

use crate::ModSource;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SourceConfig {
    url: String,
    name: String,
    description: String,
    plugin_path: String,
    
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            plugin_path: "".to_string(),
        }
    }
}

impl SourceConfig {
    pub fn from_toml(toml: &str) -> Option<Self> {
        let config: SourceConfig = toml::from_str(toml).ok()?;
        Some(config)
    }

    pub fn from_file(file: &str) -> Option<Self> {
        let toml = std::fs::read_to_string(file).ok()?;
        Self::from_toml(&toml)
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn plugin_path(&self) -> &str {
        &self.plugin_path
    }

    pub fn to_mod_source(&self) -> ModSource {
        ModSource::new(self.clone())
    }
}
