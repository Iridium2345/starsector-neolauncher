use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    name: String,
    version: String,
    url: String,
    date: String,
    author: String,
}

impl ModInfo {
    pub fn new(name: String, version: String, url: String, date: String, author: String) -> Self {
        Self {
            name,
            version,
            url,
            date,
            author,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn author(&self) -> &str {
        &self.author
    }
}