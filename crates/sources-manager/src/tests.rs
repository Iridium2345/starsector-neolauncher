use std::{env, path::Path};

use crate::{SourceConfig, Plugin};

#[test]
fn test_source_config() {
    println!("{}", env::current_dir().unwrap().display());
    let config = SourceConfig::from_file("test/config.toml").unwrap();
    assert_eq!(config.url(), "https://example.com");
    assert_eq!(config.name(), "Example");
    assert_eq!(config.description(), Some("This is an example"));
    assert_eq!(config.mod_info_selector(), "li.portal_mod_list_item");
    assert_eq!(config.mod_list_selector(), "li.portal_mod_list_item");
}

#[test]
fn test_plugin_loader() {
    let plugin = Plugin::load_from(Path::new("test"), "example").unwrap();
    let mods = plugin.enum_mods("enum_mods", "test").unwrap();
    assert_eq!(mods.len(), 1);
    assert_eq!(mods[0].name(), "test");
}
