use anyhow::{anyhow, Context};
use clap::{Arg, ArgAction, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};
use time::OffsetDateTime;

use crate::config::Config;
use crate::templating::{get_templates, EMPTY_TEMPLATE_NAME};
use crate::utils::parse_metadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNote {
    pub config: Config,
    pub template: String,
    pub name: String,
    pub name_template: String,
    pub now: String,
    pub meta: HashMap<String, String>,
}

impl NewNote {
    pub fn try_new(config: &Config, sub_matches: &ArgMatches) -> anyhow::Result<NewNote> {
        let meta = parse_metadata(sub_matches.get_many::<String>("meta-data"));
        let template = sub_matches
            .get_one::<String>("template")
            .unwrap_or(&EMPTY_TEMPLATE_NAME.to_string())
            .to_string();
        let name = sub_matches
            .get_one::<String>("name")
            .ok_or(anyhow!("No name specified"))?
            .to_string();
        let name_template = sub_matches
            .get_one::<String>("name-template")
            .unwrap_or(&"{{name}}".to_string())
            .to_string();

        Ok(NewNote {
            config: config.clone(),
            template,
            name,
            name_template,
            now: OffsetDateTime::now_utc().to_string(),
            meta: meta.into_iter().chain(config.meta.clone()).collect(),
        })
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

    pub fn cmd() -> Command {
        Command::new("new")
        .about("Creates note from specified template")
        .arg(
            Arg::new("name")
                .required(true)
                .help("Name of the note"),
        )
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE_NAME")
                .help("Handlebars template file to be used. Ex. given template file: ./template/my-template.hbs Flag should look like this: --template my-template "),
        )
        .arg(
            Arg::new("name-template")
                .short('n')
                .long("name-template")
                .value_name("TEMPLATE_STRING")
                .help("Handlebars template string for name. Ex. --name_template {{date now}}_{{name}}"),
        )
        .arg(
            Arg::new("meta-data")
                .short('m')
                .long("meta-data")
                .value_name("KEY:VALUE")
                .action(ArgAction::Append)
                .help("Key value to be passed to template. Ex. --meta-data name:John"),
        )
    }
}
