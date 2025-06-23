use sxd_document::parser::Parser;
use sxd_document::Document;
use sxd_xpath::{Context, XPath};

use crate::mod_downloader::ModInfo;
use crate::source_config::SourceConfig;

#[repr(C)]
#[derive(Debug)]
pub struct ModSource {
    config: SourceConfig,
    http_client: reqwest::Client,
    mod_list: Vec<ModInfo>,
    cached_document: Option<Document>
}

impl ModSource {
    pub fn from_config(config: SourceConfig) -> Result<Self> {
        let mut mod_source = ModSource {
            config,
            http_client: reqwest::Client::new(),
            mod_list: Vec::new(),
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

    pub async fn fetch(&mut self) -> Result<()> {
        let response = self.http_client.get(self.config.url()).send().await?;
        let body = response.text().await?;
        let document = Parser::from_str(&body).build::<Document>();
        self.cached_document = Some(document);
        Ok(())
    }

    pub async fn search(&mut self) -> Result<()> {
        match self.cached_document {
            Some(document) => {
                self.mod_list = Self::enum_mods(&document);
            }
            None => {
                self.fetch().await?;
                self.search().await
            }
        }
        Ok(())
    }

    fn enum_mods(document: &Document, config: &SourceConfig) -> Vec<ModInfo> {
        let mut mod_list = Vec::new();
        let xpath =  config;
    }
}