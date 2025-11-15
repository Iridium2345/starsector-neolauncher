use game_launcher::launch_config::LaunchConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("setting/log4rs.toml", Default::default()).unwrap_or_else(|err| {
        eprintln!("\x1b[31mWarning - Unable to initialize log configuration: {}\x1b[0m", err);
    });

    let config = LaunchConfig::from_file("setting/startup.toml")?;
    log::debug!("Game launcher: {:?}", config);
    let launcher = config.as_launcher()?;
    launcher.launch()?;
    Ok(())
}
