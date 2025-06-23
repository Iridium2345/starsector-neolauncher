use crate::SourceConfig;
use reqwest::Body;
use serde::{Deserialize, Serialize};

// 获取mod列表的函数类型 参数为html内容 返回值为mod列表
type GetModListFunc = fn(String) -> Vec<ModInfo>;

#[derive(Debug)]
pub  struct ModSource {
    config: SourceConfig,
    http_client: reqwest::Client,
    mod_list: Vec<ModInfo>,
    get_mod_list_func: GetModListFunc,
    get_mod_info_func: GetModListFunc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    name: String,
    version: String,
    url: String,
    date: String,
    author: String,
}

impl ModSource {

    fn default_get_mod_list_func(_data: String) -> Vec<ModInfo> {
        let mut mod_list = Vec::new();
        mod_list.push(ModInfo {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            url: "https://example.com".to_string(),
            date: "2021-01-01".to_string(),
            author: "test".to_string(),
        });
        mod_list
    }

    pub fn new(config: SourceConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
            mod_list: Vec::new(),
            get_mod_list_func: Self::default_get_mod_list_func,
            get_mod_info_func: Self::default_get_mod_list_func
        }
    }

    pub fn config(&self) -> &SourceConfig {
        &self.config
    }

    pub async fn search(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .http_client
            .get(self.config.url())
            .send()
            .await?;
        let body = response.text().await?;
        let mod_list = (self.get_mod_list_func)(body.clone());
        self.mod_list = mod_list;
        Ok(())
    }

    pub fn list_mod_info(&self) -> &Vec<ModInfo> {
        &self.mod_list
    }
}

impl Default for ModInfo {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            version: "".to_string(),
            url: "".to_string(),
            date: "".to_string(),
            author: "".to_string(),
        }
    }
}

impl ModInfo {
    pub fn new(name: String, version: String, url: String, date: String, author: String) -> Self {
        Self { name, version, url, date, author }
    }
}