use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command;

#[test]
fn can_run_a_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("test-echo")
        .arg("hello");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));

    Ok(())
}
