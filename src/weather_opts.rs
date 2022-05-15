use futures::future::join;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::{format_string, Error};

#[cfg(feature = "cli")]
use tokio::io::{stdout, AsyncWriteExt};

use crate::{
    config::Config, latitude::Latitude, longitude::Longitude, weather_api::WeatherLocation,
    ApiStringType, StringType,
};

#[cfg(feature = "cli")]
use crate::weather_api::WeatherApi;

/// Utility to retreive and format weather data from openweathermap.org
///
/// Please specify one of `zipcode(country_code)`, `city_name`, or `lat` and
/// `lon`.
#[cfg(feature = "cli")]
#[derive(StructOpt, Default, Serialize, Deserialize)]
pub struct WeatherOpts {
    /// Zipcode (optional)
    #[structopt(short, long)]
    zipcode: Option<u64>,
    /// Country Code (optional), if not specified `us` will be assumed
    #[structopt(short, long)]
    country_code: Option<StringType>,
    /// City Name (optional)
    #[structopt(long)]
    city_name: Option<StringType>,
    /// Latitude (must also specify Longitude)
    #[structopt(long)]
    lat: Option<Latitude>,
    /// Longitude (must also specify Latitude)
    #[structopt(long)]
    lon: Option<Longitude>,
    /// Api key (optional but either this or API_KEY environment variable must
    /// exist)
    #[structopt(short = "k", long)]
    api_key: Option<ApiStringType>,
    /// Print forecast
    #[serde(default)]
    #[structopt(short, long)]
    forecast: bool,
}

#[cfg(feature = "cli")]
impl WeatherOpts {
    /// Parse options from stdin, requires `Config` instance.
    /// # Errors
    ///
    /// Returns error if call to retreive weather data fails or if write to
    /// stdout fails
    pub async fn parse_opts(config: &Config) -> Result<(), Error> {
        let mut opts = Self::from_args();
        opts.apply_defaults(config);

        let mut stdout = stdout();
        for output in opts.run_opts(config).await? {
            stdout.write_all(output.as_bytes()).await?;
        }
        Ok(())
    }

    /// # Errors
    /// Return Error if api key cannot be found
    #[cfg(feature = "cli")]
    fn get_api(&self, config: &Config) -> Result<WeatherApi, Error> {
        let api_key = self
            .api_key
            .as_deref()
            .ok_or_else(|| Error::InvalidInputError(Self::api_help_msg().into()))?;
        Ok(WeatherApi::new(
            api_key,
            &config.api_endpoint,
            &config.api_path,
        ))
    }

    /// Extract options from `WeatherOpts` and apply to `WeatherApi`
    /// # Errors
    /// Returns Error if clap help output fails
    pub fn get_location(&self) -> Result<WeatherLocation, Error> {
        let loc = if let Some(zipcode) = self.zipcode {
            if let Some(country_code) = &self.country_code {
                WeatherLocation::from_zipcode_country_code_str(zipcode, country_code)
            } else {
                WeatherLocation::from_zipcode(zipcode)
            }
        } else if let Some(city_name) = &self.city_name {
            WeatherLocation::from_city_name(city_name)
        } else {
            if let Some(lat) = self.lat {
                if let Some(lon) = self.lon {
                    return Ok(WeatherLocation::from_lat_lon(lat, lon));
                }
            }
            Self::clap().print_help()?;
            return Err(Error::InvalidInputError(format_string!(
                "\n\nERROR: You must specify at least one option"
            )));
        };
        Ok(loc)
    }

    /// # Errors
    ///
    /// Returns error if call to retreive weather data fails
    async fn run_opts(&self, config: &Config) -> Result<Vec<StringType>, Error> {
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
        let mut output = vec![data.get_current_conditions()];
        if let Some(forecast) = forecast {
            output.extend(forecast.get_forecast());
        }
        Ok(output)
    }

    fn apply_defaults(&mut self, config: &Config) {
        if self.api_key.is_none() {
            self.api_key = config.api_key.clone();
        }
        if self.zipcode.is_none()
            && self.country_code.is_none()
            && self.city_name.is_none()
            && (self.lat.is_none() || self.lon.is_none())
        {
            self.zipcode = config.zipcode;
            self.country_code = config.country_code.clone();
            self.city_name = config.city_name.clone();
            if config.lat.is_some() && config.lon.is_some() {
                self.lat = config.lat;
                self.lon = config.lon;
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
    use isocountry::CountryCode;
    use std::{convert::TryFrom, env::set_var};

    use crate::{
        config::{Config, TestEnvs},
        latitude::Latitude,
        longitude::Longitude,
        weather_api::WeatherLocation,
        Error,
    };

    #[cfg(feature = "cli")]
    use crate::weather_opts::WeatherOpts;

    #[cfg(feature = "cli")]
    #[test]
    fn test_get_api() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);

        set_var("API_KEY", "1234567");
        set_var("API_ENDPOINT", "test.local1");
        set_var("ZIPCODE", "8675309");
        set_var("API_PATH", "weather/");

        let config = Config::init_config(None)?;
        drop(_env);

        let mut opts = WeatherOpts::default();
        opts.apply_defaults(&config);
        let api = opts.get_api(&config)?;
        assert_eq!(
            format!("{api:?}"),
            "WeatherApi(key=1234567,endpoint=test.local1)".to_string()
        );

        let loc = opts.get_location()?;
        assert_eq!(
            format!("{loc:?}"),
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

        let config = Config::init_config(None)?;
        drop(_env);

        let mut opts = WeatherOpts::default();
        opts.apply_defaults(&config);

        assert_eq!(opts.lat, Some(Latitude::try_from(10.1)?));
        assert_eq!(opts.lon, Some(Longitude::try_from(11.1)?));
        Ok(())
    }

    #[cfg(feature = "cli")]
    #[tokio::test]
    async fn test_run_opts() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);

        let config = Config::init_config(None)?;
        drop(_env);

        let mut opts = WeatherOpts::default();
        opts.zipcode = Some(55427);
        opts.apply_defaults(&config);

        let output = opts.run_opts(&config).await?;

        assert_eq!(output.len(), 1);
        assert!(output[0].contains("Current conditions Minneapolis"));

        opts.forecast = true;
        let output = opts.run_opts(&config).await?;
        assert!(output.len() == 7 || output.len() == 8);
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
