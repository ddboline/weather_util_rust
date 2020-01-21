use anyhow::Error;

use weather_util_rust::config::Config;
use weather_util_rust::weather_opts::WeatherOpts;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::init_config()?;

    WeatherOpts::parse_opts(&config).await?;
    Ok(())
}
