use anyhow::Error;
use std::io::stdout;

use weather_util_rust::config::Config;
use weather_util_rust::weather_opts::WeatherOpts;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::init_config()?;

    let (data, forecast) = WeatherOpts::parse_opts(&config).await?;
    let stdout = stdout();
    data.get_current_conditions(&mut stdout.lock())?;
    forecast.get_forecast(&mut stdout.lock())?;
    Ok(())
}
