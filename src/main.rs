pub mod config;
pub mod weather_data;
pub mod weather_opts;

use anyhow::Error;
use config::Config;
use weather_opts::WeatherOpts;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::init_config()?;

    let (data, forecast) = WeatherOpts::parse_opts(&config).await?;
    println!("{}", data.get_current_conditions());
    println!("\nForecast:");
    println!("{}", forecast.get_forecast_str());
    Ok(())
}

// ?zip={zip code},{country code}
