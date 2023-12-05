use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command;

#[test]
fn can_list_notes() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config-ls"]).arg("ls");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-note.md"));

    Ok(())
}
