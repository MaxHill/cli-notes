use crate::Config;
use serde::Deserialize;
use serde::Serialize;
use std::os::unix::process::CommandExt;

use clap::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListNotes {
    config: Config,
}

impl ListNotes {
    #[tracing::instrument]
    pub fn new(config: &Config) -> Self {
        ListNotes {
            config: config.clone(),
        }
    }

    #[tracing::instrument]
    pub fn run(&self) {
        std::process::Command::new("ls")
            .arg("-1")
            .arg(&self.config.notes_dir)
            .exec();
    }

    #[tracing::instrument]
    pub fn cmd() -> Command {
        Command::new("ls").about("List all notes")
    }
}
