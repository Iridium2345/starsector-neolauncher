use mod_downloader::SourceConfig;
use std::env;

fn main() {
    log4rs::init_file("setting/log4rs.yaml", Default::default()).unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        log::error!("使用方法: {} <模组URL>", args[0]);
        std::process::exit(1);
    }

    let config = SourceConfig::from_file(&args[1]).unwrap();
    log::info!("Mod source: {:?}", config);
}
