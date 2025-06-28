use sources_manager::{source_config::SourceConfig, ModSource};
use game_launcher::launch_config::LaunchConfig;

use std::path::Path;
use tokio;
use clap::{Parser, ValueEnum};

const PLUGIN_DIR: &str = "target/debug/";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum RunType {
    ModManager,
    GameLauncher,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    run_type: RunType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("setting/log4rs.yaml", Default::default()).unwrap_or_else(|err| {
        eprintln!("\x1b[31mWarning - Unable to initialize log configuration: {}\x1b[0m", err);
    });

    let args = Args::parse();
    match args.run_type {
        RunType::ModManager => {
            let config = SourceConfig::from_file("setting/sources/fossic.toml")?;
            log::info!("Mod source: {:?}", config);
            let mut mod_source = ModSource::from_config(config, Path::new(PLUGIN_DIR))?;
            mod_source.fetch().await?;
            mod_source.search().await?;
            log::info!("Mod list: {:?}", mod_source.mod_list());
            Ok(())
        }
        RunType::GameLauncher => {
            let config = LaunchConfig::from_file("setting/startup.yaml")?;
            log::debug!("Game launcher: {:?}", config);
            let launcher = config.as_launcher()?;
            launcher.launch()?;
            Ok(())
        }
    }
}
