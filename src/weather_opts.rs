use anyhow::{format_err, Error};
use reqwest::Url;
use structopt::StructOpt;

use crate::config::Config;
use crate::weather_data::{WeatherData, WeatherForecast};

#[derive(StructOpt)]
pub struct WeatherOpts {
    #[structopt(short, long)]
    zipcode: Option<u64>,
    #[structopt(short, long)]
    country_code: Option<String>,
    #[structopt(long)]
    city_name: Option<String>,
    #[structopt(long)]
    lat: Option<f64>,
    #[structopt(long)]
    lon: Option<f64>,
}

impl WeatherOpts {
    pub async fn parse_opts(config: &Config) -> Result<(WeatherData, WeatherForecast), Error> {
        let opts = WeatherOpts::from_args();

        let options = if let Some(zipcode) = opts.zipcode {
            let country_code = opts.country_code.unwrap_or_else(|| "us".to_string());
            vec![
                ("zip", zipcode.to_string()),
                ("country_code", country_code),
                ("APPID", config.api_key.to_string()),
            ]
        } else if let Some(city_name) = opts.city_name {
            vec![("q", city_name), ("APPID", config.api_key.to_string())]
        } else if opts.lat.is_some() & opts.lon.is_some() {
            let lat = opts.lat.unwrap().to_string();
            let lon = opts.lon.unwrap().to_string();
            vec![
                ("lat", lat),
                ("lon", lon),
                ("APPID", config.api_key.to_string()),
            ]
        } else {
            return Err(format_err!("Invalid options"));
        };

        let data: WeatherData = run_api(config, "weather", &options).await?;
        let forecast: WeatherForecast = run_api(config, "forecast", &options).await?;

        Ok((data, forecast))
    }
}

async fn run_api<T: serde::de::DeserializeOwned>(
    config: &Config,
    command: &str,
    options: &[(&'static str, String)],
) -> Result<T, Error> {
    let base_url = format!("https://{}/data/2.5/{}", config.api_endpoint, command);
    let url = Url::parse_with_params(&base_url, options)?;
    let res = reqwest::get(url).await?;
    let text = res.text().await?;
    serde_json::from_str(&text).map_err(|e| {
        println!("{}", text);
        e.into()
    })
}
