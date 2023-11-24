use std::collections::HashMap;
use std::{fs::File, path::PathBuf};
use time::macros::format_description;
use time::{
    format_description::{
        parse,
        well_known::{Iso8601, Rfc2822, Rfc3339},
    },
    PrimitiveDateTime,
};

use anyhow::{anyhow, Context};
use handlebars::{handlebars_helper, Handlebars};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteFromTemplate {
    pub config: Config,
    pub template: String,
    pub name: String,
    pub name_template: String,
    pub now: String,
    pub meta: HashMap<String, String>,
}

// a helper provides format
handlebars_helper!(date: |dt: String, {fmt:str = "[year]-[month]-[day]"}| {
    let d = [
        PrimitiveDateTime::parse(&dt, &Iso8601::DEFAULT),
        PrimitiveDateTime::parse(&dt, &Rfc3339),
        PrimitiveDateTime::parse(&dt, &Rfc2822),
        PrimitiveDateTime::parse(&dt, &format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")),
        PrimitiveDateTime::parse(&dt, &format_description!("[year]-[month]-[day] [hour]:[minute]")),
        PrimitiveDateTime::parse(&dt, &format_description!("[year]-[month]-[day] [hour]")),
    ].iter().find_map(|dt| dt.ok()).ok_or(anyhow!("Could not parse date {}", dt)).unwrap();

    // let dt: i64 = dt.parse().unwrap();
    // let dt = OffsetDateTime::from_unix_timestamp(dt).unwrap();
    d.format(&parse(fmt).unwrap()).unwrap()
}
);
static EMPTY_TEMPLATE_NAME: &str = "empty";

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
        let templates = self.templates()?;
        let data = serde_json::to_value(self)?;

        let mut file = templates
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

        let templates = self.templates()?;

        let data = serde_json::to_value(&self).context("Could not serialize note")?;
        templates
            .render_to_write(&self.template, &data, &mut output_file)
            .with_context(|| {
                format!(
                    "Could not write template {} to file {:?}",
                    self.template, output_file_path
                )
            })?;

        Ok(output_file_path)
    }

    fn templates(&self) -> anyhow::Result<Handlebars> {
        let mut handlebars = Handlebars::new();

        handlebars.register_helper("date", Box::new(date));

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

        Ok(handlebars)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn can_parse_iso_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("date", Box::new(date));

        let data = json!({"timestamp": "2023-01-01T16:00"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);

        let data = json!({"timestamp": "2023-01-01T16:00"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);
    }

    #[test]
    fn can_parse_rfc3339_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("date", Box::new(date));

        let data = json!({"timestamp": "2023-01-01T16:00:00+02:00"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);
    }

    #[test]
    fn can_parse_rfc2822_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("date", Box::new(date));

        let data = json!({"timestamp": "Mon, 1 Jan 2023 16:00:00 GMT"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);
    }

    #[test]
    fn can_parse_custom_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("date", Box::new(date));

        let data = json!({"timestamp": "2023-01-01 16:00:00"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);
    }

    #[test]
    fn can_parse_custom2_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("date", Box::new(date));

        let data = json!({"timestamp": "2023-01-01 16:00"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);
    }

    #[test]
    fn can_parse_custom3_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("date", Box::new(date));

        let data = json!({"timestamp": "2023-01-01 16"});
        let t = handlebars
            .render_template("{{date timestamp}}", &data)
            .unwrap();
        assert_eq!("2023-01-01", t);
    }
}
