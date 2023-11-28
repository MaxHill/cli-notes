mod subcommand_test;

use anyhow::Context;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::{fmt::Display, fs, path::PathBuf, process::Command};
use uuid::Uuid; // Used for writing assertions // Run programs

fn unique_file_name<N, E>(name: N, ext: E) -> String
where
    N: Into<String> + Display,
    E: std::convert::AsRef<std::ffi::OsStr>,
{
    let mut name = PathBuf::from(format!("{}-{}", name, Uuid::new_v4()));
    name.set_extension(ext);

    name.to_string_lossy().to_string()
}

fn setup_notes_dir() -> anyhow::Result<PathBuf> {
    // Path must match with ../test-config/config.toml
    let notes_dir: PathBuf = PathBuf::from("/tmp/cli-notes-test-dir");
    fs::create_dir_all(&notes_dir)?;
    Ok(notes_dir)
}

#[test]
fn can_create_a_note_from_a_template() -> Result<(), Box<dyn std::error::Error>> {
    let name = unique_file_name("test_name", "md");
    let note_path = setup_notes_dir()?.join(&name);

    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("new")
        .arg(&name)
        .args(["--template", "test-template"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&name));

    let contents = fs::read_to_string(&note_path)
        .with_context(|| format!("Could not: read file {:?}", &note_path))
        .unwrap();
    assert!(predicate::str::contains("Hello Max Hill!").eval(&contents));

    Ok(fs::remove_file(&note_path)?)
}

#[test]
fn can_create_a_note_with_a_name_template() -> Result<(), Box<dyn std::error::Error>> {
    let name = unique_file_name("test_name", "md");
    let note_output = setup_notes_dir()?;
    let expected_output_filename = note_output.clone().join(format!("test_template_{}", name));

    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("new")
        .arg(&name)
        .args(["--name-template", "test_template_{{name}}"]);

    cmd.assert().success().stdout(predicate::str::contains(
        expected_output_filename.to_string_lossy(),
    ));

    Ok(fs::remove_file(expected_output_filename)?)
}

#[test]
fn can_create_a_note_without_a_template() -> Result<(), Box<dyn std::error::Error>> {
    let name = unique_file_name("test_name", "md");
    let note_path = setup_notes_dir()?.join(&name);

    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("new")
        .arg(&name);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(note_path.to_string_lossy()));

    let contents = fs::read_to_string(&note_path)
        .with_context(|| format!("Could not: read file {:?}", &note_path))
        .unwrap();
    assert_eq!("", contents);

    Ok(fs::remove_file(&note_path)?)
}

#[test]
fn can_use_the_default_filetype() -> Result<(), Box<dyn std::error::Error>> {
    let name = unique_file_name("test_name", "");
    let note_path = setup_notes_dir()?.join(&name);

    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("new")
        .arg(&name);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(note_path.to_string_lossy()));

    let contents = fs::read_to_string(format!("{}.md", note_path.to_string_lossy()))
        .with_context(|| format!("Could not: read file {:?}", &note_path))
        .unwrap();
    assert_eq!("", contents);

    Ok(fs::remove_file(format!(
        "{}.md",
        note_path.to_string_lossy()
    ))?)
}

#[test]
fn can_use_format_dates() -> Result<(), Box<dyn std::error::Error>> {
    let name = unique_file_name("test_name", "md");
    let note_path = setup_notes_dir()?.join(&name);

    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("new")
        .arg(&name)
        .args(["--template", "test-template"]);

    cmd.assert().success();

    let contents = fs::read_to_string(&note_path)
        .with_context(|| format!("Could not: read file {:?}", &note_path))
        .unwrap();
    assert!(predicate::str::contains("Date formatted: 02/01/2023").eval(&contents));

    Ok(fs::remove_file(&note_path)?)
}

#[test]
fn can_use_metadata_flat_in_template() -> Result<(), Box<dyn std::error::Error>> {
    let name = unique_file_name("test_name", "md");
    let note_path = setup_notes_dir()?.join(&name);

    let mut cmd = Command::cargo_bin("notes-cli")?;
    cmd.args(["--config-path", "./test-config"])
        .arg("new")
        .arg(&name)
        .args(["--template", "test-meta"])
        .args(["--meta-data", "test_key_1:test-value1"])
        .args(["--meta-data", "test_key_2:test-value2"]);

    cmd.assert().success();

    let contents = fs::read_to_string(&note_path)
        .with_context(|| format!("Could not: read file {:?}", &note_path))
        .unwrap();
    assert!(predicate::str::contains("test-value1").eval(&contents));
    assert!(predicate::str::contains("test-value2").eval(&contents));
    assert!(predicate::str::contains("Max Hill").eval(&contents));

    Ok(fs::remove_file(&note_path)?)
}
