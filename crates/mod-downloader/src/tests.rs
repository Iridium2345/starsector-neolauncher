use crate::{SourceConfig, PluginLoader};

#[test]
fn test_source_config() {
    let config = SourceConfig::from_file("test/config.toml").unwrap();
    assert_eq!(config.url(), "https://example.com");
    assert_eq!(config.name(), "Example");
    assert_eq!(config.description(), "This is an example");
}



#[test]
fn test_plugin_loader() {
    #[cfg(windows)]
    let loader = PluginLoader::new("test","plugin").unwrap();

    println!("loader: {:?}", loader);
    let mods = loader.call_function("get_mods", "https://example.com").unwrap();
    assert_eq!(mods.len(), 1);
    // assert_eq!(mods[0].name, "Test Mod");
    // assert_eq!(mods[0].version, "1.0.0");
    // assert_eq!(mods[0].url, "https://example.com");
    // assert_eq!(mods[0].date, "2021-01-01");
    // assert_eq!(mods[0].author, "Test Author");
}