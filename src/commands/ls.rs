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
    pub fn new(config: &Config) -> Self {
        ListNotes {
            config: config.clone(),
        }
    }

    pub fn run(&self) {
        std::process::Command::new("ls")
            .arg("-1")
            .arg(&self.config.notes_dir)
            .exec();
    }

    pub fn cmd() -> Command {
        Command::new("ls").about("List all notes")
    }
}
