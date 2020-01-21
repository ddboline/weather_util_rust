use anyhow::{format_err, Error};
use futures::future::join;
use reqwest::Url;
use structopt::StructOpt;
use std::io::stdout;

use crate::config::Config;
use crate::latitude::Latitude;
use crate::longitude::Longitude;
use crate::weather_data::WeatherData;
use crate::weather_forecast::WeatherForecast;

/// Utility to retreive and format weather data from openweathermap.org
///
/// Please specify one of zipcode(country_code), city_name, or lat and lon.
#[derive(StructOpt, Default)]
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
    /// Api key (optional but either this or API_KEY environemnt variable must exist)
    #[structopt(short = "k", long)]
    api_key: Option<String>,
    /// Print forecast
    #[structopt(short, long)]
    forecast: bool,
}

macro_rules! set_default {
    ($s:ident, $c:ident, $id:ident) => {
        $s.$id = $c.$id.clone();
    };
}

impl WeatherOpts {
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

    /// Return `WeatherData` and `WeatherForecast` by parsing Stdin
    pub async fn parse_opts(config: &Config) -> Result<(), Error> {
        let mut opts = Self::from_args();
        opts.apply_defaults(config);
        let (data, forecast) = opts.process_opts(config).await?;
        let stdout = stdout();
        data.get_current_conditions(&mut stdout.lock())?;
        if opts.forecast {
            forecast.get_forecast(&mut stdout.lock())?;
        }
        Ok(())
    }

    fn api_help_msg() -> String {
        let config_dir = dirs::config_dir().expect("This shouldn't happen");
        format!(
            "API_KEY environment variable must be set\nEither set them directly or place them in {}",
            config_dir.join("weather_util").join("config.env").to_string_lossy()
        )
    }

    /// Process `WeatherOpts` options
    pub async fn process_opts(
        &self,
        config: &Config,
    ) -> Result<(WeatherData, WeatherForecast), Error> {
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| format_err!(Self::api_help_msg()))?;
        let api_endpoint = config
            .api_endpoint
            .clone()
            .unwrap_or_else(|| "api.openweathermap.org".to_string());

        let options = self.get_options(api_key)?;
        let (data, forecast) = join(
            run_api(&api_endpoint, "weather", &options),
            run_api(&api_endpoint, "forecast", &options),
        )
        .await;

        let data: WeatherData = data?;
        let forecast: WeatherForecast = forecast?;

        Ok((data, forecast))
    }

    fn get_options(&self, api_key: &str) -> Result<Vec<(&'static str, String)>, Error> {
        let options = if let Some(zipcode) = self.zipcode {
            let country_code = self
                .country_code
                .clone()
                .unwrap_or_else(|| "us".to_string());
            vec![
                ("zip", zipcode.to_string()),
                ("country_code", country_code),
                ("APPID", api_key.to_string()),
            ]
        } else if let Some(city_name) = self.city_name.clone() {
            vec![("q", city_name), ("APPID", api_key.to_string())]
        } else if self.lat.is_some() & self.lon.is_some() {
            let lat = self.lat.unwrap().to_string();
            let lon = self.lon.unwrap().to_string();
            vec![("lat", lat), ("lon", lon), ("APPID", api_key.to_string())]
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
    api_endpoint: &str,
    command: &str,
    options: &[(&'static str, String)],
) -> Result<T, Error> {
    let base_url = format!("https://{}/data/2.5/{}", api_endpoint, command);
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
