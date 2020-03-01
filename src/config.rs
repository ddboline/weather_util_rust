use anyhow::{format_err, Error};
use std::{env::var, ops::Deref, path::Path, sync::Arc};

use crate::{latitude::Latitude, longitude::Longitude};

/// Configuration data
#[derive(Default, Debug)]
pub struct ConfigInner {
    /// openweathermap.org api key
    pub api_key: Option<String>,
    /// openweathermap.org api endpoint
    pub api_endpoint: Option<String>,
    /// Api path (default is `data/2.5/`)
    pub api_path: Option<String>,
    /// optional default zipcode
    pub zipcode: Option<u64>,
    /// optional default country code
    pub country_code: Option<String>,
    /// optional default city name
    pub city_name: Option<String>,
    /// optional default latitude
    pub lat: Option<Latitude>,
    /// optional default longitude
    pub lon: Option<Longitude>,
}

/// Configuration struct
#[derive(Default, Debug, Clone)]
pub struct Config(Arc<ConfigInner>);

macro_rules! set_config_ok {
    ($s:ident, $id:ident) => {
        $s.$id = var(&stringify!($id).to_uppercase()).ok();
    };
}

macro_rules! set_config_parse {
    ($s:ident, $id:ident) => {
        $s.$id = var(&stringify!($id).to_uppercase())
            .ok()
            .and_then(|x| x.parse().ok());
    };
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pull in configuration data using `[dotenv](https://crates.io/dotenv)`.
    ///
    /// If a .env file exists in the current directory, pull in any ENV variables in it.
    ///
    /// Next, if a config file exists in the current directory named config.env,
    /// or if a config file exists at `${HOME}/.config/weather_util/config.env`,
    /// set ENV variables using it.
    ///
    /// Config files should have lines of the following form: `API_KEY=api_key_value`
    ///
    /// # Example
    ///
    /// ```
    /// # use std::env::{set_var, var};
    /// use weather_util_rust::config::Config;
    /// use anyhow::Error;
    ///
    /// # fn main() -> Result<(), Error> {
    /// # set_var("API_KEY", "api_key_value");
    /// # set_var("API_ENDPOINT", "api.openweathermap.org");
    /// let config = Config::init_config()?;
    /// assert_eq!(config.api_key, var("API_KEY").ok());
    /// assert_eq!(config.api_endpoint, var("API_ENDPOINT").ok());
    /// # Ok(())
    /// # }
    /// ```
    pub fn init_config() -> Result<Self, Error> {
        let fname = Path::new("config.env");
        let config_dir = dirs::config_dir().ok_or_else(|| format_err!("No CONFIG directory"))?;
        let default_fname = config_dir.join("weather_util").join("config.env");

        let env_file = if fname.exists() {
            fname
        } else {
            &default_fname
        };

        dotenv::dotenv().ok();

        if env_file.exists() {
            dotenv::from_path(env_file).ok();
        }

        let mut conf = ConfigInner::default();

        set_config_ok!(conf, api_key);
        set_config_ok!(conf, api_endpoint);
        set_config_ok!(conf, api_path);
        set_config_parse!(conf, zipcode);
        set_config_ok!(conf, country_code);
        set_config_ok!(conf, city_name);
        set_config_parse!(conf, lat);
        set_config_parse!(conf, lon);

        Ok(Self(Arc::new(conf)))
    }
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
