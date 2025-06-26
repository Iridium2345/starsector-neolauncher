use std::path::Path;

use crate::mod_info::ModInfo;
use crate::source_config::SourceConfig;
use crate::plugin_loader::Plugin;

#[repr(C)]
#[derive(Debug)]
pub struct ModSource{
    config: SourceConfig,
    http_client: reqwest::Client,
    mod_list: Vec<ModInfo>,
    cached_document: Option<String>,
    plugin: Plugin,
}

impl ModSource {
    pub fn from_config(config: SourceConfig, plugin_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let plugin = Plugin::load_from(plugin_dir, &config.plugin())?;
        let mod_source = ModSource {
            config,
            http_client: reqwest::Client::new(),
            mod_list: Vec::new(),
            cached_document: None,
            plugin: plugin,
        };
        Ok(mod_source)
    }

    pub fn config(&self) -> &SourceConfig {
        &self.config
    }

    pub fn mod_list(&self) -> &Vec<ModInfo> {
        &self.mod_list
    }

    pub fn mod_list_as_mut(&mut self) -> &mut Vec<ModInfo> {
        &mut self.mod_list
    }

    pub async fn fetch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Fetching mod list from {}", self.config.url());
        let response = self.http_client.get(self.config.url()).send().await?;
        let body = response.text().await?;
        log::trace!("body: {}", body);
        self.cached_document = Some(body);
        Ok(())
    }

    pub async fn search(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.cached_document {
            Some(package) => {
                self.mod_list = self.enum_mods(&package, &self.config)?;
            }
            None => {
                self.fetch().await?;
                if let Some(package) = &self.cached_document {
                    self.mod_list = self.enum_mods(&package, &self.config)?;
                } else {
                    return Err("Cached document is None".into());
                }
            }
        }
        Ok(())
    }

    fn enum_mods(&self, package: &str, config: &SourceConfig) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>> {
        let mod_list = self.plugin.enum_mods(&config.mod_list_selector(), package)?;
        Ok(mod_list)
    }
}