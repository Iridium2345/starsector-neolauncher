use std::path::{Path, PathBuf};
use libloading::Library;

use crate::ModInfo;

#[cfg(target_os = "windows")]
const PLUGIN_EXTENSION: &str = ".dll";

#[cfg(target_os = "linux")]
const PLUGIN_EXTENSION: &str = ".so";

#[cfg(target_os = "macos")]
const PLUGIN_EXTENSION: &str = ".dylib";

#[repr(C)]
#[derive(Debug)]
pub struct Plugin {
    path: PathBuf,
    name: String,
    library: Library,
}

impl Plugin {

    fn plugin_path(path: &Path, name: &str) -> PathBuf{
        path.join(format!("lib{}{}", name, PLUGIN_EXTENSION))
    }

    pub fn load_from(path: &Path, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let plugin_path = Self::plugin_path(path, name);
        let library = unsafe { Library::new(plugin_path.as_path())? };
        Ok(Self { path: plugin_path, name: name.to_string(), library })
    }

    pub fn enum_mods(&self, func_name: &str, html: &str) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>> {
        let mods = unsafe {
            let func: libloading::Symbol<fn(&str) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>>> = self.library.get(func_name.as_bytes())?;
            func(html)?
        };
        Ok(mods)
    }
}