use anyhow::{anyhow, Context};
use handlebars::handlebars_helper;
use handlebars::Handlebars;
use time::macros::format_description;
use time::{
    format_description::{
        parse,
        well_known::{Iso8601, Rfc2822, Rfc3339},
    },
    PrimitiveDateTime,
};

use crate::config::Config;

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

pub static EMPTY_TEMPLATE_NAME: &str = "empty";

pub fn get_templates(config: &Config) -> anyhow::Result<Handlebars> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("date", Box::new(date));

    handlebars
        .register_template_string(EMPTY_TEMPLATE_NAME, "")
        .with_context(|| "Could not register empty template".to_string())?;

    handlebars
        .register_templates_directory(".hbs", &config.templates_dir)
        .with_context(|| {
            format!(
                "Could not register teamplate directory {:?}",
                config.templates_dir
            )
        })?;

    Ok(handlebars)
}

#[cfg(test)]
mod test {
    use super::*;
    use handlebars::Handlebars;
    use serde_json::json;

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
