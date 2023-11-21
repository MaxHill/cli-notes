use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub editor: String,
    pub notes_dir: String,
    pub templates_dir: String,

    pub subcommands: HashMap<String, String>,
    pub meta: HashMap<String, String>,
}

pub fn get_config(config_dir: &str) -> anyhow::Result<Config> {
    let filename = format!("{}config.toml", config_dir);
    let contents = fs::read_to_string(&filename)
        .with_context(|| format!("Could not find config file: {:?}", filename))?;
    toml::from_str::<Config>(&contents)
        .with_context(|| format!("Could not construct Config from {:?}", contents))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_can_get_config_from_file() {
        match get_config("./test-config/") {
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
        match get_config("./does-not-exist/") {
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
