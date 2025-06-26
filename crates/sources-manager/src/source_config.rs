use std::fmt::Debug;
use serde::{Deserialize,Serialize};

#[repr(C)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    url: String,
    name: String,
    description: Option<String>,
    plugin: String,
    #[serde(alias = "mod-list-selector")]
    mod_list_selector: String,
    #[serde(alias = "mod-info-selector")]
    mod_info_selector: String,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            name: "".to_string(),
            plugin: "".to_string(),
            description: None,
            mod_list_selector: "".to_string(),
            mod_info_selector: "".to_string(),
        }
    }
}

impl SourceConfig {
    pub fn from_toml(toml: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: SourceConfig = toml::from_str(toml)?;
        Ok(config)
    }

    pub fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let toml = std::fs::read_to_string(file)?;
        Self::from_toml(&toml)
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn plugin(&self) -> &str {
        &self.plugin
    }

    pub fn mod_list_selector(&self) -> &str {
        &self.mod_list_selector
    }

    pub fn mod_info_selector(&self) -> &str {
        &self.mod_info_selector
    }
}
