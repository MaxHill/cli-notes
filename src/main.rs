pub mod config;
pub mod note_from_template;

use std::collections::HashMap;

use crate::{config::get_config_path, note_from_template::NoteFromTemplate};
use anyhow::anyhow;
use clap::{command, parser::ValuesRef, Arg, ArgAction, Command};
use config::Config;

fn cmd() -> Command {
    command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("config-path")
                .short('c')
                .long("config-path")
                .help("Provide a path to the config directory. Default is $XDG_CONFIG_HOME"),
        )
        .subcommand(new())
}

fn new() -> Command {
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

fn parse_metadata(meta: Option<ValuesRef<'_, String>>) -> HashMap<String, String> {
    meta.unwrap_or_default()
        .filter_map(|v| {
            v.split_once(':')
                .map(|(key, value)| (key.to_string(), value.to_string()))
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let matches = cmd().get_matches();

    let config_path = get_config_path(matches.get_one::<String>("config-path"))?;
    let config = Config::new_from_path(config_path)?;

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let meta = parse_metadata(sub_matches.get_many::<String>("meta-data"));
            let template = sub_matches.get_one::<String>("template");
            let name = sub_matches
                .get_one::<String>("name")
                .ok_or(anyhow!("No name specified"))?;
            let name_template = sub_matches.get_one::<String>("name-template");
            // let created_file = create_from_template(&config, "test-template", "test-template.md")?;
            let note_from_template =
                NoteFromTemplate::new(&config, template, name.to_string(), name_template, meta);
            let created_file = note_from_template.write()?;

            println!("{}", created_file.display());
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn can_parse_metadata() {
        let m = Command::new("test")
            .arg(Arg::new("test").short('t').action(ArgAction::Append))
            .get_matches_from(vec![
                "test",
                "-t",
                "arg1",
                "-t",
                "k_one:v_one",
                "-t",
                "k_two:v_two",
            ]);

        assert_eq!(
            HashMap::from([
                ("k_one".to_string(), "v_one".to_string()),
                ("k_two".to_string(), "v_two".to_string())
            ]),
            parse_metadata(m.get_many("test"))
        )
    }
}
