use clap::{command, Arg, ArgAction, Command};
use commands::{ls::ListNotes, new_note::NewNote, subcommand::SubCommand};
use config::Config;

mod commands;
mod config;
mod templating;
mod utils;

#[tracing::instrument]
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
                .help("Provide a path to the config directory. Default is $XDG_CONFIG_HOME/notes-cli/cofnig.toml"),
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
        .subcommand(ListNotes::cmd())
}

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let matches = cmd().get_matches();

    let config = Config::try_new(&matches)?;

    match matches.subcommand() {
        Some(("new", sub_matches)) => println!(
            "{}",
            NewNote::try_new(&config, sub_matches)?.write()?.display()
        ),
        Some(("ls", _)) => ListNotes::new(&config).run(),
        Some(matching) => SubCommand::try_new(&config, matching)?.run()?,
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
