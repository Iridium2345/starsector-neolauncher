use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize,Serialize};

#[repr(C)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    url: String,
    name: String,
    #[serde(default = "unknown")]
    description: String,
    #[serde(alias = "mod-list-xpath")]
    mod_list_xpath: String,
    #[serde(alias = "mod-info-xpath")]
    mod_info_xpath: String,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            mod_list_xpath: "".to_string(),
            mod_info_xpath: "".to_string(),
            client_config: HashMap::new()
        }
    }
}

impl SourceConfig {
    pub fn from_toml(toml: &str) -> Result<Self> {
        let config: SourceConfig = toml::from_str(toml)?;
        Ok(config)
    }

    pub fn from_file(file: &str) -> Result<Self> {
        let toml = std::fs::read_to_string(file)?;
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

    pub fn mod_list_xpath(&self) -> &str {
        &self.mod_list_xpath
    }

    pub fn mod_info_xpath(&self) -> &str {
        &self.mod_info_xpath
    }

    pub fn client_config(&self) -> &HashMap<String,String> {
        &self.client_config
    }

}
