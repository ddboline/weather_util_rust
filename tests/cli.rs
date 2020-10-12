use anyhow::Error;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn test_default() -> Result<(), Error> {
    let mut cmd = Command::cargo_bin("weather-util-rust")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Current conditions"));

    Ok(())
}
