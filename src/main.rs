use std::{
    collections::HashMap,
    fs::{self, File},
};

use anyhow::Context;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    editor: String,
    notes_dir: String,
    templates_dir: String,

    subcommands: HashMap<String, String>,
    meta: HashMap<String, String>,
}

fn main() -> anyhow::Result<()> {
    let config = get_config("./test-config/")?;
    let file_path = create_from_template(&config, "test-template", "test-template.md")?;

    println!("Created: {}", file_path);
    Ok(())
}

fn get_config(config_dir: &str) -> anyhow::Result<Config> {
    let filename = format!("{}config.toml", config_dir);
    let contents = fs::read_to_string(&filename)
        .with_context(|| format!("Could not find config file: {:?}", filename))?;
    toml::from_str::<Config>(&contents)
        .with_context(|| format!("Could not construct Config from {:?}", contents))
}

fn create_from_template(config: &Config, template: &str, name: &str) -> anyhow::Result<String> {
    let output_file_path = config.notes_dir.to_owned() + name;

    let mut handlebars = Handlebars::new();

    let mut output_file = File::create(&output_file_path)
        .with_context(|| format!("Could not create file or directory {:?}", output_file_path))?;

    handlebars
        .register_templates_directory(".hbs", &config.templates_dir)
        .with_context(|| {
            format!(
                "Could not register teamplate directory {}",
                config.templates_dir
            )
        })?;

    let data = serde_json::to_value(config).context("Could not serialize config")?;
    handlebars
        .render_to_write(template, &data, &mut output_file)
        .with_context(|| {
            format!(
                "Could not write template {} to file {:?}",
                template, output_file_path
            )
        })?;

    Ok(output_file_path)
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
