use crate::new_note::NewNote;
use clap::{command, Arg, ArgAction, Command};
use config::Config;
use subcommand::SubCommand;

pub mod config;
pub mod new_note;
pub mod subcommand;
pub mod templating;
pub mod utils;

pub fn cmd() -> Command {
    command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("config-path")
                .short('c')
                .long("config-path")
                .help("Provide a path to the config directory. Default is $XDG_CONFIG_HOME"),
        )
        .arg(
            Arg::new("meta-data")
                .short('m')
                .long("meta-data")
                .value_name("KEY:VALUE")
                .action(ArgAction::Append)
                .help("Aditional key value pairs to be added to config. Ex. --meta-data name:John"),
        )
        .subcommand(NewNote::cmd())
}

fn main() -> anyhow::Result<()> {
    let matches = cmd().get_matches();

    let config = Config::try_new(&matches)?;

    match matches.subcommand() {
        Some(("new", sub_matches)) => println!(
            "{}",
            NewNote::try_new(&config, sub_matches)?.write()?.display()
        ),
        Some(matching) => SubCommand::try_new(&config, matching)?.run()?,
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
