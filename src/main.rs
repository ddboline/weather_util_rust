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
    println!("{:?}", data);
    println!("{}", data.get_current_conditions());
    println!("{:?}", forecast.city);
    for l in &forecast.list {
        println!("{}", l.get_forecast_entry(forecast.city.timezone));
    }
    Ok(())
}

// ?zip={zip code},{country code}
