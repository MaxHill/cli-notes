use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::File, path::PathBuf};
use time::OffsetDateTime;

use crate::config::Config;
use crate::templating::{get_templates, EMPTY_TEMPLATE_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteFromTemplate {
    pub config: Config,
    pub template: String,
    pub name: String,
    pub name_template: String,
    pub now: String,
    pub meta: HashMap<String, String>,
}

impl NoteFromTemplate {
    pub fn new(
        config: &Config,
        template: Option<&String>,
        name: String,
        name_template: Option<&String>,
        meta: HashMap<String, String>,
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
            now: OffsetDateTime::now_utc().to_string(),
            meta: meta.into_iter().chain(config.meta.clone()).collect(),
        }
    }

    fn get_file_name(&self) -> anyhow::Result<PathBuf> {
        let data = serde_json::to_value(self)?;

        let mut file = get_templates(&self.config)?
            .render_template(&self.name_template, &data)
            .map(PathBuf::from)?;
        file.set_extension(&self.config.note_file_type);

        Ok(file)
    }

    pub fn write(self) -> anyhow::Result<PathBuf> {
        let output_file_path = PathBuf::from(&self.config.notes_dir).join(self.get_file_name()?);

        let mut output_file = File::create(&output_file_path).with_context(|| {
            format!("Could not create file or directory {:?}", output_file_path)
        })?;

        let data = serde_json::to_value(&self).context("Could not serialize note")?;
        get_templates(&self.config)?
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
