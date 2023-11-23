use std::{fs::File, path::PathBuf};

use anyhow::Context;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteFromTemplate {
    pub config: Config,
    pub template: String,
    pub name: String,
    pub name_template: String,
}

static EMPTY_TEMPLATE_NAME: &str = "empty";

impl NoteFromTemplate {
    pub fn new(
        config: &Config,
        template: Option<&String>,
        name: String,
        name_template: Option<&String>,
    ) -> Self {
        let template = template
            .unwrap_or(&EMPTY_TEMPLATE_NAME.to_string())
            .to_string();
        let name_template = name_template.unwrap_or(&"{{name}}".to_string()).to_string();

        NoteFromTemplate {
            config: config.to_owned(),
            template,
            name,
            name_template,
        }
    }

    fn get_file_name(&self) -> anyhow::Result<PathBuf> {
        let data = serde_json::to_value(self)?;

        let mut file = Handlebars::new()
            .render_template(&self.name_template, &data)
            .map(PathBuf::from)?;
        file.set_extension(&self.config.note_file_type);

        Ok(file)
    }

    pub fn write(self) -> anyhow::Result<PathBuf> {
        let output_file_path = PathBuf::from(&self.config.notes_dir).join(self.get_file_name()?);
        println!("{:?}", self.get_file_name());
        let mut handlebars = Handlebars::new();

        let mut output_file = File::create(&output_file_path).with_context(|| {
            format!("Could not create file or directory {:?}", output_file_path)
        })?;

        handlebars
            .register_template_string(EMPTY_TEMPLATE_NAME, "")
            .with_context(|| "Could not register empty template".to_string())?;

        handlebars
            .register_templates_directory(".hbs", &self.config.templates_dir)
            .with_context(|| {
                format!(
                    "Could not register teamplate directory {:?}",
                    self.config.templates_dir
                )
            })?;

        let data = serde_json::to_value(self.config).context("Could not serialize config")?;
        handlebars
            .render_to_write(&self.template, &data, &mut output_file)
            .with_context(|| {
                format!(
                    "Could not write template {} to file {:?}",
                    self.template, output_file_path
                )
            })?;

        Ok(output_file_path)
    }
}
