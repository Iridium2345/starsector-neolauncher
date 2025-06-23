use mod_downloader::mod_src::ModInfo;

#[unsafe(no_mangle)]
pub fn get_mods(html: &str) -> Vec<ModInfo> {
    let mut mods = vec![];
    mods.push(ModInfo::new("Test Mod".to_string(), "1.0.0".to_string(), "https://example.com".to_string(), "2021-01-01".to_string(), "Test Author".to_string()));
    mods
}