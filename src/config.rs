use anyhow::Context;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
};

use crate::utils::{parse_metadata, parse_metadata_json};

fn default_file_md() -> String {
    "md".to_string()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_file_md")]
    pub note_file_type: String,
    pub editor: Option<String>,
    pub notes_dir: PathBuf,
    pub templates_dir: PathBuf,

    pub subcommands: HashMap<String, String>,
    pub meta: HashMap<String, String>,
}

impl Config {
    pub fn try_new(matches: &ArgMatches) -> anyhow::Result<Config> {
        let config_path = get_config_path(matches.get_one::<String>("config-path"))?;
        let mut additional_metadata = parse_metadata(matches.get_many::<String>("meta-data"));
        let additional_metadata_json =
            parse_metadata_json(matches.get_many::<String>("meta-data-json"));
        for (key, value) in additional_metadata_json {
            additional_metadata.insert(key, value);
        }

        let filename = config_path.join("config.toml");

        let contents = fs::read_to_string(&filename)
            .with_context(|| format!("Could not find config file: {:?}", filename))?;

        toml::from_str::<Config>(&contents)
            .with_context(|| format!("Could not construct Config from {:?}", contents))
            .map(|c| c.clone_add_meta(additional_metadata))
    }

    pub fn clone_add_meta(&self, meta: HashMap<String, String>) -> Config {
        let mut c = self.clone();
        c.meta = meta.into_iter().chain(self.meta.clone()).collect();
        c.clone()
    }
}

pub fn get_config_path(flag: Option<&String>) -> anyhow::Result<PathBuf> {
    let path_flag = flag.map(|c| Path::new(c).to_path_buf()).ok_or(());

    match path_flag {
        Ok(path) => Ok(path),
        Err(_) => {
            let xdg_config_home = std::env::var("XDG_CONFIG_HOME")
                .context("Could not find env variable XDG_CONFIG_HOME")?;
            Ok(Path::new(&xdg_config_home).join("notes-cli"))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use clap::{Arg, ArgAction, Command};
    use predicates::{prelude::predicate, Predicate};

    fn get_matches_from<S>(dir: S, flags: Option<Vec<&str>>) -> clap::ArgMatches
    where
        S: std::convert::AsRef<std::path::Path>,
    {
        let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir);
        let d = binding.to_str().unwrap();

        let f = [vec!["--config-path", d], flags.unwrap_or(vec![])].concat();
        println!("F: {:?}", f);

        Command::new("test")
            .no_binary_name(true)
            .arg(
                Arg::new("config-path")
                    .short('c')
                    .long("config-path")
                    .help("Provide a path to the config directory. Default is $XDG_CONFIG_HOME"),
            )
            .arg(
                Arg::new("meta-data")
                    .short('m')
                    .long("meta-data")
                    .value_name("KEY:VALUE")
                    .action(ArgAction::Append)
                    .help("Aditional key value pairs to be added to config. Ex. --meta-data name:John"),
            )
            .arg(
                Arg::new("meta-data-json")
                    .long("meta-data-json")
                    .value_name("json")
                    .action(ArgAction::Append)
                    .help("Aditional key value pairs to be added to config, passed as json. Ex. --meta-data-json \"{\"name\": \"John\"}\""),)
            .get_matches_from(f)
    }

    #[test]
    fn it_can_get_config_from_file() {
        match Config::try_new(&get_matches_from("test-config", None)) {
            Ok(config) => {
                assert_eq!(config.notes_dir, PathBuf::from("/tmp/cli-notes-test-dir"));
            }
            Err(e) => {
                panic!("Error when getting config: {}", e);
            }
        };
    }

    #[test]
    fn it_fails_if_config_does_not_exist() {
        match Config::try_new(&get_matches_from("does-not-exist", None)) {
            Ok(config) => {
                panic!("Did not error when it should have: {:?}", config);
            }
            Err(e) => {
                assert!(
                    predicate::str::contains("Could not find config file:").eval(&e.to_string())
                );
                assert!(predicate::str::contains("does-not-exist/config.toml").eval(&e.to_string()));
            }
        };
    }

    #[test]
    fn adds_metadata_to_configs_metadata() {
        match Config::try_new(&get_matches_from(
            "test-config",
            Some(vec!["--meta-data", "some:value"]),
        )) {
            Ok(config) => {
                assert_eq!("value", config.meta.get("some").unwrap());
            }
            Err(e) => {
                panic!("Error when getting config: {}", e);
            }
        };
    }

    #[test]
    fn adds_metadata_to_configs_metadata_in_json_format() {
        match Config::try_new(&get_matches_from(
            "test-config",
            Some(vec!["--meta-data-json", "{\"some\": \"value\"}"]),
        )) {
            Ok(config) => {
                assert_eq!("value", config.meta.get("some").unwrap());
            }
            Err(e) => {
                panic!("Error when getting config: {}", e);
            }
        };
    }
}
