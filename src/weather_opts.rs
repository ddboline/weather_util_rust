use anyhow::{format_err, Error};
use futures::future::join;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::io::{stdout, AsyncWriteExt};
use structopt::StructOpt;

use crate::{
    config::Config,
    latitude::Latitude,
    longitude::Longitude,
    weather_api::{WeatherApi, WeatherLocation},
    weather_data::WeatherData,
    weather_forecast::WeatherForecast,
};

/// Utility to retreive and format weather data from openweathermap.org
///
/// Please specify one of `zipcode(country_code)`, `city_name`, or `lat` and
/// `lon`.
#[derive(StructOpt, Default, Serialize, Deserialize)]
pub struct WeatherOpts {
    /// Zipcode (optional)
    #[structopt(short, long)]
    zipcode: Option<u64>,
    /// Country Code (optional), if not specified `us` will be assumed
    #[structopt(short, long)]
    country_code: Option<String>,
    /// City Name (optional)
    #[structopt(long)]
    city_name: Option<String>,
    /// Latitude (must also specify Longitude)
    #[structopt(long)]
    lat: Option<Latitude>,
    /// Longitude (must also specify Latitude)
    #[structopt(long)]
    lon: Option<Longitude>,
    /// Api key (optional but either this or API_KEY environment variable must
    /// exist)
    #[structopt(short = "k", long)]
    api_key: Option<String>,
    /// Print forecast
    #[serde(default)]
    #[structopt(short, long)]
    forecast: bool,
}

macro_rules! set_default {
    ($s:ident, $c:ident, $id:ident) => {
        $s.$id = $c.$id.clone();
    };
}

impl WeatherOpts {
    /// Parse options from stdin, requires `Config` instance.
    pub async fn parse_opts(config: &Config) -> Result<(), Error> {
        let mut opts = Self::from_args();
        opts.apply_defaults(config);
        opts.run_opts(config).await?;
        Ok(())
    }

    fn get_api(&self, config: &Config) -> Result<WeatherApi, Error> {
        let api_key = self
            .api_key
            .as_deref()
            .ok_or_else(|| format_err!(Self::api_help_msg()))?;
        let api_endpoint = config
            .api_endpoint
            .as_deref()
            .unwrap_or("api.openweathermap.org");
        let api_path = config.api_path.as_deref().unwrap_or("data/2.5/");
        Ok(WeatherApi::new(api_key, api_endpoint, api_path))
    }

    /// Extract options from `WeatherOpts` and apply to `WeatherApi`
    pub fn get_location(&self) -> Result<WeatherLocation, Error> {
        let loc = if let Some(zipcode) = self.zipcode {
            if let Some(country_code) = &self.country_code {
                WeatherLocation::from_zipcode_country_code(zipcode, country_code)
            } else {
                WeatherLocation::from_zipcode(zipcode)
            }
        } else if let Some(city_name) = &self.city_name {
            WeatherLocation::from_city_name(city_name)
        } else if self.lat.is_some() && self.lon.is_some() {
            let lat = self.lat.unwrap();
            let lon = self.lon.unwrap();
            WeatherLocation::from_lat_lon(lat, lon)
        } else {
            Self::clap().print_help()?;
            return Err(format_err!(
                "\n\nERROR: You must specify at least one option"
            ));
        };
        Ok(loc)
    }

    async fn run_opts(&self, config: &Config) -> Result<(), Error> {
        let api = self.get_api(config)?;
        let loc = self.get_location()?;

        let data = api.get_weather_data(&loc);
        let (data, forecast) = if self.forecast {
            let forecast = api.get_weather_forecast(&loc);
            let (data, forecast) = join(data, forecast).await;
            (data?, Some(forecast?))
        } else {
            (data.await?, None)
        };

        let mut stdout = stdout();
        stdout.write_all(data.get_current_conditions()?.as_bytes()).await?;
        if let Some(forecast) = forecast {
            stdout.write_all(forecast.get_forecast()?.as_bytes()).await?;
        }
        Ok(())
    }

    fn apply_defaults(&mut self, config: &Config) {
        if self.api_key.is_none() {
            set_default!(self, config, api_key);
        }
        if self.zipcode.is_none()
            && self.country_code.is_none()
            && self.city_name.is_none()
            && (self.lat.is_none() || self.lon.is_none())
        {
            set_default!(self, config, zipcode);
            set_default!(self, config, country_code);
            set_default!(self, config, city_name);
            if config.lat.is_some() && config.lon.is_some() {
                set_default!(self, config, lat);
                set_default!(self, config, lon);
            }
        }
    }

    fn api_help_msg() -> String {
        let config_dir = dirs::config_dir().expect("This shouldn't happen");
        format!(
            "API_KEY environment variable must be set\nEither set them directly or place them in \
             {}",
            config_dir
                .join("weather_util")
                .join("config.env")
                .to_string_lossy()
        )
    }
}
