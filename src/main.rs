use anyhow::Error;

#[cfg(feature = "cli")]
use weather_util_rust::{config::Config, weather_opts::WeatherOpts};

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::init_config()?;

    WeatherOpts::parse_opts(&config).await?;
    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() -> Result<(), Error> {
    Ok(())
}
