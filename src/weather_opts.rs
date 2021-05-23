use anyhow::{format_err, Error};
use futures::future::join;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use stack_string::StackString;
use structopt::StructOpt;
use tokio::io::{stdout, AsyncWriteExt};

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
    country_code: Option<StackString>,
    /// City Name (optional)
    #[structopt(long)]
    city_name: Option<StackString>,
    /// Latitude (must also specify Longitude)
    #[structopt(long)]
    lat: Option<Latitude>,
    /// Longitude (must also specify Latitude)
    #[structopt(long)]
    lon: Option<Longitude>,
    /// Api key (optional but either this or API_KEY environment variable must
    /// exist)
    #[structopt(short = "k", long)]
    api_key: Option<StackString>,
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

        let mut stdout = stdout();
        for output in opts.run_opts(config).await? {
            stdout.write_all(output.as_bytes()).await?;
        }
        Ok(())
    }

    fn get_api(&self, config: &Config) -> Result<WeatherApi, Error> {
        let api_key = self
            .api_key
            .as_deref()
            .ok_or_else(|| format_err!(Self::api_help_msg()))?;
        let api_endpoint = config
            .api_endpoint
            .as_ref()
            .map_or("api.openweathermap.org", AsRef::as_ref);
        let api_path = config.api_path.as_ref().map_or("data/2.5/", AsRef::as_ref);
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

    async fn run_opts(&self, config: &Config) -> Result<Vec<String>, Error> {
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
        let mut output = vec![data.get_current_conditions()?];
        if let Some(forecast) = forecast {
            output.extend(forecast.get_forecast()?);
        }
        Ok(output)
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

#[cfg(test)]
mod test {
    use anyhow::Error;
    use isocountry::CountryCode;
    use std::{
        convert::TryFrom,
        env::{set_var, var_os},
        ffi::{OsStr, OsString},
    };

    use crate::{
        config::{Config, TestEnvs},
        latitude::Latitude,
        longitude::Longitude,
        weather_api::WeatherLocation,
        weather_opts::WeatherOpts,
    };

    #[test]
    fn test_get_api() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);

        set_var("API_KEY", "1234567");
        set_var("API_ENDPOINT", "test.local1");
        set_var("ZIPCODE", "8675309");
        set_var("API_PATH", "weather/");

        let config = Config::init_config()?;
        drop(_env);

        let mut opts = WeatherOpts::default();
        opts.apply_defaults(&config);
        let api = opts.get_api(&config)?;
        assert_eq!(
            format!("{:?}", api),
            "WeatherApi(key=1234567,endpoint=test.local1)".to_string()
        );

        let loc = opts.get_location()?;
        assert_eq!(
            format!("{:?}", loc),
            "ZipCode { zipcode: 8675309, country_code: None }".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_apply_defaults() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "LAT", "LON", "API_PATH"]);

        set_var("API_KEY", "1234567");
        set_var("API_ENDPOINT", "test.local1");
        set_var("LAT", "10.1");
        set_var("LON", "11.1");
        set_var("API_PATH", "weather/");

        let config = Config::init_config()?;
        drop(_env);

        let mut opts = WeatherOpts::default();
        opts.apply_defaults(&config);

        assert_eq!(opts.lat, Some(Latitude::try_from(10.1)?));
        assert_eq!(opts.lon, Some(Longitude::try_from(11.1)?));
        Ok(())
    }

    #[tokio::test]
    async fn test_run_opts() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);

        let config = Config::init_config()?;
        drop(_env);

        let mut opts = WeatherOpts::default();
        opts.zipcode = Some(55427);
        opts.apply_defaults(&config);

        let output = opts.run_opts(&config).await?;

        assert_eq!(output.len(), 1);
        assert!(output[0].contains("Current conditions Minneapolis"));

        opts.forecast = true;
        let output = opts.run_opts(&config).await?;
        assert_eq!(output.len(), 8);
        println!("{:#?}", output);
        assert!(output[1].contains("Forecast:"));
        assert!(output[2].contains("High:"));
        assert!(output[2].contains("Low:"));

        Ok(())
    }

    #[test]
    fn test_api_help_msg() -> Result<(), Error> {
        let msg = WeatherOpts::api_help_msg();
        assert!(msg.len() > 0);
        Ok(())
    }

    #[test]
    fn test_get_location() -> Result<(), Error> {
        let mut opts = WeatherOpts::default();
        opts.zipcode = Some(55427);
        opts.country_code = Some("US".into());
        let loc = opts.get_location()?;
        assert_eq!(
            loc,
            WeatherLocation::ZipCode {
                zipcode: 55427,
                country_code: CountryCode::for_alpha2("US").ok(),
            }
        );

        let mut opts = WeatherOpts::default();
        opts.city_name = Some("Pittsburgh".into());
        let loc = opts.get_location()?;
        assert_eq!(loc, WeatherLocation::CityName("Pittsburgh".into()));

        let mut opts = WeatherOpts::default();
        opts.lat = Latitude::try_from(11.1).ok();
        opts.lon = Longitude::try_from(12.2).ok();

        let loc = opts.get_location()?;
        assert_eq!(
            loc,
            WeatherLocation::LatLon {
                latitude: Latitude::try_from(11.1)?,
                longitude: Longitude::try_from(12.2)?
            }
        );

        let opts = WeatherOpts::default();
        assert!(opts.get_location().is_err());

        Ok(())
    }
}
