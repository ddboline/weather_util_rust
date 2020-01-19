use anyhow::{format_err, Error};
use futures::future::join;
use reqwest::Url;
use structopt::StructOpt;

use crate::config::Config;
use crate::latitude::Latitude;
use crate::longitude::Longitude;
use crate::weather_data::WeatherData;
use crate::weather_forecast::WeatherForecast;

/// Utility to retreive and format weather data from openweathermap.org
#[derive(StructOpt, Default)]
pub struct WeatherOpts {
    /// Zipcode (optional)
    #[structopt(short, long)]
    zipcode: Option<u64>,
    /// Country Code (optional), if not specified `us` will be assumed
    #[structopt(short, long)]
    country_code: Option<String>,
    /// City Name
    #[structopt(long)]
    city_name: Option<String>,
    /// Latitude (must also specify Longitude)
    #[structopt(long)]
    lat: Option<Latitude>,
    /// Longitude (must also specify Latitude)
    #[structopt(long)]
    lon: Option<Longitude>,
}

impl WeatherOpts {
    /// Return WeatherData and WeatherForecast by parsing Stdin
    pub async fn parse_opts(config: &Config) -> Result<(WeatherData, WeatherForecast), Error> {
        let opts = Self::from_args();
        opts.process_opts(config).await
    }

    /// Process WeatherOpts options
    pub async fn process_opts(
        &self,
        config: &Config,
    ) -> Result<(WeatherData, WeatherForecast), Error> {
        let options = self.get_options(config)?;
        let (data, forecast) = join(
            run_api(config, "weather", &options),
            run_api(config, "forecast", &options),
        )
        .await;

        let data: WeatherData = data?;
        let forecast: WeatherForecast = forecast?;

        Ok((data, forecast))
    }

    fn get_options(&self, config: &Config) -> Result<Vec<(&'static str, String)>, Error> {
        let options = if let Some(zipcode) = self.zipcode {
            let country_code = self
                .country_code
                .clone()
                .unwrap_or_else(|| "us".to_string());
            vec![
                ("zip", zipcode.to_string()),
                ("country_code", country_code),
                ("APPID", config.api_key.to_string()),
            ]
        } else if let Some(city_name) = self.city_name.clone() {
            vec![("q", city_name), ("APPID", config.api_key.to_string())]
        } else if self.lat.is_some() & self.lon.is_some() {
            let lat = self.lat.unwrap().to_string();
            let lon = self.lon.unwrap().to_string();
            vec![
                ("lat", lat),
                ("lon", lon),
                ("APPID", config.api_key.to_string()),
            ]
        } else {
            Self::clap().print_help()?;
            return Err(format_err!(
                "\n\nERROR: You must specify at least one option"
            ));
        };
        Ok(options)
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

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::config::Config;
    use crate::weather_opts::WeatherOpts;

    #[tokio::test]
    #[ignore]
    async fn test_process_opts() -> Result<(), Error> {
        let opts = WeatherOpts {
            zipcode: Some(11106),
            ..WeatherOpts::default()
        };
        let config = Config::init_config()?;
        let (data, forecast) = opts.process_opts(&config).await?;
        assert!(data.name == "Astoria", format!("{:?}", data));
        assert!(forecast.city.timezone == -18000, format!("{:?}", forecast));
        Ok(())
    }
}
