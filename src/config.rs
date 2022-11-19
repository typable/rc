use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub editor: String,
    pub programs: Option<HashMap<String, String>>,
}

impl Config {
    pub fn get_config_path() -> Result<String> {
        let path = get_config_path()?;
        let file_path = get_file_path(&path);
        Ok(file_path.display().to_string())
    }

    pub fn load() -> Result<Self> {
        let path = get_config_path()?;
        let file_path = get_file_path(&path);
        if !file_path.exists() {
            let config = create_config_file(&path, &file_path)?;
            return Ok(config);
        }
        let content = fs::read_to_string(&file_path)?;
        let config = toml::de::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = get_config_path()?;
        let file_path = get_file_path(&path);
        let content = toml::ser::to_string(&self)?;
        fs::write(&file_path, content)?;
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf> {
    let mut path = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(ErrorKind::NoConfigPath.into()),
    };
    path.push(APP_NAME);
    Ok(path)
}

fn get_file_path(path: &Path) -> PathBuf {
    let mut file_path = path.to_path_buf();
    file_path.push(CONFIG_FILE);
    file_path
}

fn create_config_file(path: &PathBuf, file_path: &PathBuf) -> Result<Config> {
    let mut programs = HashMap::new();
    programs.insert("rc".to_string(), file_path.display().to_string());
    let config = Config {
        editor: env::var("EDITOR").unwrap_or_else(|_| "vim".to_string()),
        programs: Some(programs),
    };
    let content = toml::ser::to_string(&config)?;
    fs::create_dir_all(path)?;
    fs::write(file_path, content)?;
    warn!(
        "A new config file was created at '{}'.",
        file_path.display()
    );
    Ok(config)
}
