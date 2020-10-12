use anyhow::{format_err, Error};
use serde::Deserialize;
use std::env::{set_var, var_os};
use std::ffi::{OsStr, OsString};
use std::{ops::Deref, path::Path, sync::Arc};

use crate::{latitude::Latitude, longitude::Longitude};

/// Configuration data
#[derive(Default, Debug, Deserialize, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Config(Arc<ConfigInner>);

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pull in configuration data using `[dotenv](https://crates.io/dotenv)`.
    ///
    /// If a .env file exists in the current directory, pull in any ENV
    /// variables in it.
    ///
    /// Next, if a config file exists in the current directory named config.env,
    /// or if a config file exists at `${HOME}/.config/weather_util/config.env`,
    /// set ENV variables using it.
    ///
    /// Config files should have lines of the following form:
    /// `API_KEY=api_key_value`
    ///
    /// # Example
    ///
    /// ```
    /// # use std::env::set_var;
    /// use weather_util_rust::config::Config;
    /// use anyhow::Error;
    ///
    /// # fn main() -> Result<(), Error> {
    /// # set_var("API_KEY", "api_key_value");
    /// # set_var("API_ENDPOINT", "api.openweathermap.org");
    /// let config = Config::init_config()?;
    /// assert_eq!(config.api_key, Some("api_key_value".into()));
    /// assert_eq!(config.api_endpoint, Some("api.openweathermap.org".into()));
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

        let conf: ConfigInner = envy::from_env()?;

        Ok(Self(Arc::new(conf)))
    }
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct TestEnvs {
    envs: Vec<(OsString, OsString)>,
}

impl TestEnvs {
    pub(crate) fn new(keys: &[impl AsRef<OsStr>]) -> Self {
        Self {
            envs: keys
                .iter()
                .filter_map(|k| var_os(k).map(|v| (k.as_ref().to_os_string(), v)))
                .collect(),
        }
    }
}

impl Drop for TestEnvs {
    fn drop(&mut self) {
        for (key, val) in &self.envs {
            set_var(key, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use std::env::set_var;

    use crate::config::{Config, TestEnvs};

    #[test]
    fn test_config() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);

        assert_eq!(Config::new(), Config::default());

        set_var("API_KEY", "1234567");
        set_var("API_ENDPOINT", "test.local");
        set_var("ZIPCODE", "8675309");
        set_var("API_PATH", "weather/");

        let conf = Config::init_config()?;
        drop(_env);
        assert_eq!(conf.api_key, Some("1234567".to_string()));
        assert_eq!(conf.api_endpoint, Some("test.local".to_string()));
        assert_eq!(conf.zipcode, Some(8675309));
        assert_eq!(conf.api_path, Some("weather/".to_string()));
        Ok(())
    }
}
