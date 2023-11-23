use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn default_file_md() -> String {
    "md".to_string()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_file_md")]
    pub note_file_type: String,
    pub editor: String,
    pub notes_dir: PathBuf,
    pub templates_dir: PathBuf,

    pub subcommands: HashMap<String, String>,
    pub meta: HashMap<String, String>,
}

impl Config {
    // pub fn new_from_path(config_dir_path: PathBuf) -> anyhow::Result<Config>
    pub fn new_from_path<P>(config_path: P) -> anyhow::Result<Config>
    where
        std::path::PathBuf: std::convert::From<P>,
    {
        let filename = PathBuf::from(config_path).join("config.toml");

        let contents = fs::read_to_string(&filename)
            .with_context(|| format!("Could not find config file: {:?}", filename))?;
        toml::from_str::<Config>(&contents)
            .with_context(|| format!("Could not construct Config from {:?}", contents))
    }
}
pub fn get_config_path(flag: Option<&String>) -> anyhow::Result<PathBuf> {
    let path_flag = flag.map(|c| Path::new(c).to_path_buf()).ok_or(());

    match path_flag {
        Ok(path) => Ok(path),
        Err(_) => {
            let xdg_config_home = std::env::var("XDG_CONFIG_HOME")
                .context("Could not find env variable XDG_CONFIG_HOME")?;
            Ok(Path::new(&xdg_config_home).join("/notes-cli"))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_can_get_config_from_file() {
        match Config::new_from_path("./test-config/") {
            Ok(config) => {
                assert_eq!(config.editor, "vim");
            }
            Err(e) => {
                panic!("Error when getting config: {}", e);
            }
        };
    }

    #[test]
    fn it_fails_if_config_does_not_exist() {
        match Config::new_from_path("./does-not-exist/") {
            Ok(config) => {
                panic!("Did not error when it should have: {:?}", config);
            }
            Err(e) => {
                assert_eq!(
                    format!("{}", e),
                    "Could not find config file: \"./does-not-exist/config.toml\""
                );
            }
        };
    }
}
