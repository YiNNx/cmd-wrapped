use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

lazy_static::lazy_static! {
    static ref PATH_CONFIG: String=format!(
        "{}/.local/share/cmd-wrapped/config.toml", env::var("HOME").expect("cannot get $HOME")
    );
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub providers: Vec<Provider>,
}

#[derive(Deserialize, Serialize)]
pub struct Provider {
    pub name: String,
    pub history_files: Vec<String>,
}

impl Config {
    pub fn read() -> Result<Option<Config>, Box<dyn Error>> {
        let mut str = String::new();
        let config_path = Path::new(PATH_CONFIG.as_str());
        if !config_path.is_file() {
            return Ok(None);
        }
        fs::File::open(config_path)?.read_to_string(&mut str)?;
        let config = toml::from_str(&str)?;
        Ok(Some(config))
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let str = toml::to_string(&self)?;
        let config_path = Path::new(PATH_CONFIG.as_str());
        let dir = config_path
            .parent()
            .expect("failed to get local config path");
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        fs::write(config_path, str)?;
        Ok(())
    }
}
