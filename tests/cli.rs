use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
                            // use std::env;
use std::process::Command; // Run programs

#[test]
fn invalid_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-whoami")?;

    cmd.arg("-a");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '-a' found",
    ));
    Ok(())
}

#[test]
fn long_help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-whoami")?;

    cmd.arg("--help");
    let contains_predicate = (predicate::str::contains("-h, --help     Print help"))
        .and(predicate::str::contains("-V, --version  Print version"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn short_help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-whoami")?;

    cmd.arg("-h");
    let contains_predicate = (predicate::str::contains("-h, --help     Print help"))
        .and(predicate::str::contains("-V, --version  Print version"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn long_version_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-whoami")?;

    cmd.arg("--version");
    let contains_predicate = predicate::str::contains("rust-whoami 0.1.0");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn short_version_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-whoami")?;

    cmd.arg("-V");
    let contains_predicate = predicate::str::contains("rust-whoami 0.1.0");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}
