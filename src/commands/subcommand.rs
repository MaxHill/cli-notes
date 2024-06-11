use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{ffi::OsString, os::unix::process::CommandExt};

use clap::ArgMatches;

use crate::{config::Config, templating};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubCommand {
    config: Config,
    cmd: String,
    args: String,
}

impl SubCommand {
    #[tracing::instrument]
    pub fn try_new(
        config: &Config,
        (subcommand, sub_matches): (&str, &ArgMatches),
    ) -> anyhow::Result<SubCommand> {
        let args = sub_matches
            .get_many::<OsString>("")
            .into_iter()
            .flatten()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        let cmd = config
            .subcommands
            .get(subcommand)
            .ok_or(anyhow!("Subcommand not found in config"))?
            .to_string();

        Ok(SubCommand {
            config: config.clone(),
            args,
            cmd,
        })
    }

    #[tracing::instrument]
    pub fn run(&self) -> anyhow::Result<()> {
        let data = serde_json::to_value(self)?;
        let cmd = templating::get_templates(&self.config)?.render_template(&self.cmd, &data)?;

        std::process::Command::new("sh").arg("-c").arg(cmd).exec();

        Ok(())
    }
}
