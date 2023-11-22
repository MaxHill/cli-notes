pub mod config;
use std::fs::File;

use anyhow::Context;
use clap::{command, Arg, Command};
use config::Config;
use handlebars::Handlebars;

use crate::config::get_config_path;

fn cmd() -> Command {
    command!() // requires `cargo` feature
        .arg(
            Arg::new("config-path")
                .short('c')
                .long("config-path")
                .help("Provide a path to the config directory. Default is $XDG_CONFIG_HOME"),
        )
}

fn main() -> anyhow::Result<()> {
    let matches = cmd().get_matches();

    let config_path = get_config_path(matches.get_one::<String>("config-path"))?;
    let config = Config::new_from_path(config_path)?;

    let created_file = create_from_template(&config, "test-template", "test-template.md")?;

    println!("{}", created_file);
    Ok(())
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
