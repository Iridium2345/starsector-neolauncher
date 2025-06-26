use sources_manager::ModInfo;

#[unsafe(no_mangle)]
pub fn enum_mods(_html: &str) -> Result<Vec<ModInfo>, Box<dyn std::error::Error>> {
    Ok(vec![
        ModInfo::new(
            "test".to_string(), 
            "1.0.0".to_string(), 
            "https://example.com".to_string(), 
            "2021-01-01".to_string(), 
            "test".to_string())
    ])
}

