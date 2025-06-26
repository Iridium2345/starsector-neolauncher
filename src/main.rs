use sources_manager::{source_config::SourceConfig, ModSource};
use std::path::Path;
use tokio;

const PLUGIN_DIR: &str = "target/debug/";

#[tokio::main]
async fn main() {
    log4rs::init_file("setting/log4rs.yaml", Default::default()).unwrap();

    let config = SourceConfig::from_file("setting/sources/fossic.toml").unwrap();
    log::info!("Mod source: {:?}", config);
    let mut mod_source = ModSource::from_config(config, Path::new(PLUGIN_DIR)).unwrap();
    mod_source.fetch().await.unwrap();
    mod_source.search().await.unwrap();
    log::info!("Mod list: {:?}", mod_source.mod_list());
}
