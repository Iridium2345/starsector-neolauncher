use std::path::PathBuf;

use libloading::Library;
use crate::mod_src::ModInfo;



#[derive(Debug)]
pub struct PluginLoader {
    library: Library,
}

impl PluginLoader {

    #[cfg(target_os = "windows")]
    fn get_plugin_path(name:&str) -> String {
        let mut path = String::from(name);
        path.push_str(".dll");
        path
    }

    #[cfg(target_os = "linux")]
    fn get_plugin_path(name:&str) -> String {
        let mut path = String::from(name);
        path.push_str(".so");
        path
    }

    #[cfg(target_os = "macos")]
    fn get_plugin_path(name:&str) -> String {
        let mut path = String::from(name);
        path.push_str(".dylib");
        path
    }

    pub fn new(path: &str,name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut actual_path = PathBuf::from(path);
        actual_path.push(PluginLoader::get_plugin_path(name));
        let library = unsafe { Library::new(actual_path.to_str().expect(format!("Cannot get path {}",&name).as_str()))? };
        Ok(Self { library })
    }

    pub fn call_function(&self, name: &str, html:&str) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>>{
        let func = unsafe {self.library.get::<fn(&str) -> Vec<ModInfo>>(name.as_bytes())?};
        let mods = func(html);
        Ok(mods)
    }
}