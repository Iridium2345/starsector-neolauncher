use crate::LaunchConfig;

const YAML_CONTENT: &str = r#"
platform: "linux"
game_version: "0.98RC-7"
main_class: "com.fs.starfarer.StarfarerLauncher"
working_dir: "/home/test/starsector"

path:
    java: "/usr/bin/java"
    jars: "/home/test/starsector/"
    native: "/home/test/starsector/native"
    save: "/home/test/starsector/saves"
    log: "/home/test/starsector/logs"
    screenshots: "/home/test/starsector/screenshots"
    mod: "/home/test/starsector/mods"

args: 
    game: []
    jvm: 
        memory:
            stack: 4 
            min: 1024
            max: 1024
        jmodules: 
            open:                 
                java.base/sun.nio.ch: ALL-UNNAMED
            export: 
                java.base/jdk.internal.ref: ALL-UNNAMED
        jvm_args:            
            - Djava.util.Arrays.useLegacyMergeSort=true
        vm:
            - +UnlockDiagnosticVMOptions
            - TieredStopAtLevel=4
"#;

#[test]
fn test_config_loading() {

    let config = LaunchConfig::from_yaml(YAML_CONTENT).expect("Failed to parse YAML");
    
    assert_eq!(config.platform, "linux");
    assert_eq!(config.game_version, "0.98RC-7");
    assert_eq!(config.main_class, "com.fs.starfarer.StarfarerLauncher");
    assert_eq!(config.working_dir, "/home/test/starsector");
    assert_eq!(config.path.java, "/usr/bin/java");
    assert_eq!(config.path.jars, "/home/test/starsector/");
    assert_eq!(config.args.jvm.memory.min, 1024);
    assert_eq!(config.args.jvm.memory.max, 1024);
    assert_eq!(config.args.jvm.memory.stack, 4);
    
    println!("配置加载成功！");
}