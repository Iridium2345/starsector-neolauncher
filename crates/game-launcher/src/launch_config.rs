use std::{collections::HashMap, env, fs::{File, OpenOptions}, path::{Path, PathBuf}, process::Command};
use log;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
const LIB_DIR: &str = "windows";
#[cfg(target_os = "linux")]
const LIB_DIR: &str = "linux";
#[cfg(target_os = "macos")]
const LIB_DIR: &str = "macos";

const CLASSPATH_SEPARATOR: &str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    pub java: String,
    pub jars: String,
    pub native: String,
    pub save: String,
    pub log: String,
    pub screenshots: String,
    #[serde(rename = "mod")]
    pub mod_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub stack: i64,
    pub min: i64,
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JModulesConfig {
    #[serde(default)]
    pub open: HashMap<String, String>,
    #[serde(default)]
    pub export: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JvmConfig {
    pub memory: MemoryConfig,
    pub jmodules: JModulesConfig,
    pub jvm_args: Vec<String>,
    pub vm: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgsConfig {
    #[serde(default)]
    pub game: Vec<String>,
    pub jvm: JvmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchConfig {
    #[serde(skip)]
    pub file_path: Option<String>,
    pub platform: String,
    pub game_version: String,
    pub main_class: String,
    pub working_dir: String,
    pub stdout: String,
    pub stderr: String,
    pub log4j: String,
    pub path: PathConfig,
    pub args: ArgsConfig,
}

#[derive(Debug)]
pub struct Launcher {
    config: LaunchConfig,
    native_dir: PathBuf,
    libs: Vec<PathBuf>
}

impl LaunchConfig {

    #[inline]
    fn replace_env_vars(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = path.to_string();
        result = result.replace("$WORKING_DIR", &self.working_dir);
        result = result.replace("$LAUNCHER_DIR", env::current_dir()?.to_str().unwrap());
        result = result.replace("$GAME_VERSION", &self.game_version);
        Ok(result)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config: LaunchConfig = serde_yaml::from_str(yaml)?;
        config.file_path = None;        
        Ok(config)
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml = std::fs::read_to_string(path)?;
        let mut config = Self::from_yaml(&yaml)?;
        config.file_path = Some(path.to_string());
        Ok(config)
    }

    fn replace_path(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stdout = self.replace_env_vars(&self.stdout)?;
        self.stderr = self.replace_env_vars(&self.stderr)?;
        self.log4j = self.replace_env_vars(&self.log4j)?;

        self.path.java = self.replace_env_vars(&self.path.java)?;
        self.path.jars = self.replace_env_vars(&self.path.jars)?;
        self.path.native = self.replace_env_vars(&self.path.native)?;

        self.path.mod_path = self.replace_env_vars(&self.path.mod_path)?;
        self.path.save = self.replace_env_vars(&self.path.save)?;
        self.path.screenshots = self.replace_env_vars(&self.path.screenshots)?;
        self.path.log = self.replace_env_vars(&self.path.log)?;

        Ok(())
    }

    pub fn as_launcher(mut self) -> Result<Launcher, Box<dyn std::error::Error>> {
        self.replace_path()?;

        log::info!("Native path: {:?}", self.path.native);
        let native_dir = PathBuf::from(&self.path.native).join(LIB_DIR);
        log::info!("Native directory: {:?}", native_dir);
        let jar_path = Path::new(&self.path.jars);

        log::info!("Jar path: {:?}", jar_path);
        let mut libs: Vec<PathBuf> = vec![];
        for entry in jar_path.read_dir()? {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "jar") {
                    log::debug!("Jar found: {:?}", path);
                    libs.push(path);
                }
            }
        }
        log::debug!("Libs: {:?}", libs);
        
        Ok(Launcher {
            config: self,
            native_dir,
            libs
        })
    }
}

impl Launcher {
    pub fn startup_command(self) -> Result<Command, Box<dyn std::error::Error>> {
        log::info!("Generate startup command: {:?}", &self.config.file_path);
        let mut cmd = Command::new(&self.config.path.java);

        // 设置工作目录
        cmd.current_dir(&self.config.working_dir);
        
        // 添加内存配置
        cmd.arg(format!("-Xms{}m", self.config.args.jvm.memory.min));
        cmd.arg(format!("-Xmx{}m", self.config.args.jvm.memory.max));
        cmd.arg(format!("-Xss{}m", self.config.args.jvm.memory.stack));
        
        // 添加 VM 参数（以 - 开头的）
        for vm_arg in &self.config.args.jvm.vm {
            cmd.arg(format!("-XX:{}", vm_arg));
        }
        
        // 添加 JVM 参数（以 -D 开头的）
        for jvm_arg in &self.config.args.jvm.jvm_args {
            cmd.arg(format!("-{}", jvm_arg));
        }
        
        // 添加模块开放参数
        for (module, target) in &self.config.args.jvm.jmodules.open {
            cmd.arg("--add-opens");
            cmd.arg(format!("{}={}", module, target));
        }
        
        // 添加模块导出参数
        for (module, target) in &self.config.args.jvm.jmodules.export {
            cmd.arg("--add-exports");
            cmd.arg(format!("{}={}", module, target));
        }
        
        // 设置 native 库路径
        cmd.arg(format!("-Djava.library.path={}", self.native_dir.display()));
        
        // 构建 classpath
        let mut classpath = String::new();
        for (i, lib) in self.libs.iter().enumerate() {
            if i > 0 {
                classpath.push_str(CLASSPATH_SEPARATOR);
            }
            classpath.push_str(&lib.display().to_string());
        }

        cmd.arg(format!("-Dcom.fs.starfarer.settings.paths.saves={}", self.config.path.save));
        cmd.arg(format!("-Dcom.fs.starfarer.settings.paths.screenshots={}", self.config.path.screenshots));
        cmd.arg(format!("-Dcom.fs.starfarer.settings.paths.logs={}", self.config.path.log));
        cmd.arg(format!("-Dcom.fs.starfarer.settings.paths.mods={}", self.config.path.mod_path));
        cmd.arg(format!("-Dlog4j.configuration=file:{}", self.config.log4j));

        if !classpath.is_empty() {
            cmd.arg("-cp");
            cmd.arg(&classpath);
        }
        
        // 添加主类
        cmd.arg(&self.config.main_class);
        
        // 添加游戏参数
        cmd.args(self.config.args.game);
        
        Ok(cmd)
    }

    #[inline]
    fn stdio_file(path: &str) -> Result<File, std::io::Error> {
        OpenOptions::new().read(true).write(true).append(true).create(true).open(path)
    }

    pub fn launch(self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!(
            "Launching game: {} \nprofile: {:?} \nplatform: {}", 
            &self.config.working_dir, 
            &self.config.file_path.clone().unwrap_or("unknown".to_string()), 
            &self.config.platform
        );

        log::info!("config : {:?}", &self.config);
        log::info!("libs : {:?}", &self.libs);
        log::info!("native_dir : {:?}", &self.native_dir);

        let stdout = Self::stdio_file(&self.config.stdout)?;
        let stderr = Self::stdio_file(&self.config.stderr)?;

        let mut cmd = self.startup_command()?;
        log::debug!("Startup command: {:?}", &cmd);

        cmd.stdout(stdout);
        cmd.stderr(stderr);

        cmd.spawn().unwrap_or_else(|err| {
            log::error!("Failed to spawn startup command: {}", err);
            std::process::exit(1);
        });

        Ok(())
    }

    pub fn get_config(&self) -> &LaunchConfig {
        &self.config
    }

    pub fn get_libs(&self) -> &Vec<PathBuf> {
        &self.libs
    }

    pub fn get_native_dir(&self) -> &PathBuf {
        &self.native_dir
    }
}

