pub mod config;
pub mod note_from_template;

use crate::{config::get_config_path, note_from_template::NoteFromTemplate};
use anyhow::anyhow;
use clap::{command, Arg, Command};
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
                .help("Handlebars template file to be used. Ex. given template file: ./template/my-template.hbs Flag should look like this: --template my-template "),
        )
        .arg(
            Arg::new("name-template")
                .short('n')
                .long("name-template")
                .help("Handlebars template string for name"),
        )
}

fn main() -> anyhow::Result<()> {
    let matches = cmd().get_matches();

    let config_path = get_config_path(matches.get_one::<String>("config-path"))?;
    let config = Config::new_from_path(config_path)?;

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let template = sub_matches.get_one::<String>("template");
            let name = sub_matches
                .get_one::<String>("name")
                .ok_or(anyhow!("No name specified"))?;
            let name_template = sub_matches.get_one::<String>("name-template");
            // let created_file = create_from_template(&config, "test-template", "test-template.md")?;
            let note_from_template =
                NoteFromTemplate::new(&config, template, name.to_string(), name_template);
            let created_file = note_from_template.write()?;

            println!("{}", created_file.display());
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
