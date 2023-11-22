use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

/// Copy files from source to destination recursively.
// fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
//     fs::create_dir_all(&destination)?;
//     for entry in fs::read_dir(source)? {
//         let entry = entry?;
//         let filetype = entry.file_type()?;
//         if filetype.is_dir() {
//             copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
//         } else {
//             fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
//         }
//     }
//     Ok(())
// }
//
// fn setup_test_config() -> anyhow::Result<String> {
//     let config_dir: String = "/tmp/cli-notes-test-dir".to_string();
//     fs::remove_dir_all(&config_dir)?;
//     fs::create_dir_all(&config_dir)?;
//     Ok(config_dir)
// }

#[test]
fn can_create_a_note() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("notes-cli")?;

    cmd.arg("--config-path").arg("./test-config");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("/tmp/test-template.md"));

    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
