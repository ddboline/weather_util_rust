use anyhow::Error;
use assert_cmd::{Command, cargo::cargo_bin};
use log::info;

use weather_util_rust::config::TestEnvs;

#[ignore]
#[test]
fn test_default() -> Result<(), Error> {
    let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);
    let bin = cargo_bin("weather-util-rust");
    assert!(bin.exists());

    let output = Command::cargo_bin("weather-util-rust")?
        .args(["-z", "10001"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    info!("{}", stdout);
    info!("{}", stderr);

    assert!(stdout.contains("Current conditions"));

    Ok(())
}
