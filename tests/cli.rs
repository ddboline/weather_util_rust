use anyhow::Error;
use assert_cmd::Command;

#[test]
fn test_default() -> Result<(), Error> {
    let output = Command::cargo_bin("weather-util-rust")?.output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{}", stdout);
    println!("{}", stderr);

    assert!(stdout.contains("Current conditions"));

    Ok(())
}
