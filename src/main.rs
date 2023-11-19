use std::{collections::HashMap, fs::File};

use anyhow::Context;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    editor: String,
    notes_dir: String,
    templates_dir: String,

    subcommands: HashMap<String, String>,
    meta: HashMap<String, String>,
}

fn main() -> anyhow::Result<()> {
    let config = Config {
        editor: "vim".to_string(),
        notes_dir: "/tmp/".to_string(),
        templates_dir: "./test-data".to_string(),
        subcommands: HashMap::from([]),
        meta: HashMap::from([("who".to_string(), "am I really?".to_string())]),
    };

    let file_path = create_from_template(&config)?;

    println!("Created: {}", file_path);
    Ok(())
}

fn create_from_template(config: &Config) -> anyhow::Result<String> {
    let template = "test-template";
    let output_file_path = config.notes_dir.to_owned() + "test-template.md";

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
