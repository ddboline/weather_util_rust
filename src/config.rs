use std::sync::LazyLock;
use parking_lot::{Mutex, MutexGuard};
use serde::Deserialize;
use std::{
    env::{remove_var, set_var, var_os},
    ffi::{OsStr, OsString},
    ops::Deref,
    path::Path,
    sync::Arc,
};

use crate::{latitude::Latitude, longitude::Longitude, ApiStringType, Error, StringType};

/// Configuration data
#[derive(Default, Debug, Deserialize, PartialEq, Eq)]
pub struct ConfigInner {
    /// openweathermap.org api key
    pub api_key: Option<ApiStringType>,
    /// openweathermap.org api endpoint
    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: StringType,
    /// Api path (default is `data/2.5/`)
    #[serde(default = "default_api_path")]
    pub api_path: StringType,
    /// Geo Api path (default is `geo/1.0/`)
    #[serde(default = "default_geo_path")]
    pub geo_path: StringType,
    /// optional default zipcode
    pub zipcode: Option<u64>,
    /// optional default country code
    pub country_code: Option<StringType>,
    /// optional default city name
    pub city_name: Option<StringType>,
    /// optional default latitude
    pub lat: Option<Latitude>,
    /// optional default longitude
    pub lon: Option<Longitude>,
}

fn default_api_endpoint() -> StringType {
    "api.openweathermap.org".into()
}

fn default_api_path() -> StringType {
    "data/2.5/".into()
}

fn default_geo_path() -> StringType {
    "geo/1.0/".into()
}

/// Configuration struct
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Config(Arc<ConfigInner>);

impl Config {
    #[must_use]
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
    /// # use weather_util_rust::config::TestEnvs;
    /// use anyhow::Error;
    ///
    /// # fn main() -> Result<(), Error> {
    /// # let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);
    /// # set_var("API_KEY", "api_key_value");
    /// # set_var("API_ENDPOINT", "api.openweathermap.org");
    /// let config = Config::init_config(None)?;
    /// # drop(_env);
    /// assert_eq!(config.api_key, Some("api_key_value".into()));
    /// assert_eq!(&config.api_endpoint, "api.openweathermap.org");
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    ///
    /// Will return Error if unable to deserialize env variables
    pub fn init_config(config_path: Option<&Path>) -> Result<Self, Error> {
        let fname = config_path.unwrap_or_else(|| Path::new("config.env"));
        let config_dir = dirs::config_dir().unwrap_or_else(|| "./".into());
        let default_fname = config_dir.join("weather_util").join("config.env");

        let env_file = if fname.exists() {
            fname
        } else {
            &default_fname
        };

        dotenvy::dotenv().ok();

        if env_file.exists() {
            dotenvy::from_path(env_file).ok();
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

static TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub struct TestEnvs<'a> {
    _guard: MutexGuard<'a, ()>,
    envs: Vec<(OsString, Option<OsString>)>,
}

impl TestEnvs<'_> {
    #[allow(dead_code)]
    pub fn new(keys: &[impl AsRef<OsStr>]) -> Self {
        let guard = TEST_MUTEX.lock();
        let envs = keys
            .iter()
            .map(|k| (k.as_ref().to_os_string(), var_os(k)))
            .collect();
        Self {
            _guard: guard,
            envs,
        }
    }
}

impl Drop for TestEnvs<'_> {
    fn drop(&mut self) {
        for (key, val) in &self.envs {
            if let Some(val) = val {
                set_var(key, val);
            } else {
                remove_var(key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use log::info;
    use std::{
        env::{remove_var, set_var},
        fs::write,
    };
    use tempfile::NamedTempFile;

    use crate::{
        config::{Config, TestEnvs},
        Error,
    };

    #[test]
    fn test_config() -> Result<(), Error> {
        assert_eq!(Config::new(), Config::default());

        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);

        set_var("API_KEY", "fb2380d74189c9983ea52f55914da824");
        set_var("API_ENDPOINT", "test.local");
        set_var("ZIPCODE", "8675309");
        set_var("API_PATH", "weather/");

        let conf = Config::init_config(None)?;
        drop(_env);

        info!("{}", conf.api_key.as_ref().unwrap());
        assert_eq!(
            conf.api_key.as_ref().unwrap().as_str(),
            "fb2380d74189c9983ea52f55914da824"
        );

        #[cfg(feature = "stackstring")]
        assert!(conf.api_key.as_ref().unwrap().is_inline());

        assert_eq!(&conf.api_endpoint, "test.local");
        assert_eq!(conf.zipcode, Some(8675309));
        assert_eq!(&conf.api_path, "weather/");
        Ok(())
    }

    #[test]
    fn test_config_file() -> Result<(), Error> {
        let _env = TestEnvs::new(&["API_KEY", "API_ENDPOINT", "ZIPCODE", "API_PATH"]);
        remove_var("API_KEY");
        remove_var("API_ENDPOINT");
        remove_var("ZIPCODE");
        remove_var("API_PATH");
        let config_data = include_bytes!("../tests/config.env");
        let config_file = NamedTempFile::new()?;
        let config_path = config_file.path();
        write(config_file.path(), config_data)?;
        let conf = Config::init_config(Some(config_path))?;
        assert_eq!(
            conf.api_key.as_ref().unwrap().as_str(),
            "fb2380d74189c9983ea52f55914da824"
        );

        #[cfg(feature = "stackstring")]
        assert!(conf.api_key.as_ref().unwrap().is_inline());

        assert_eq!(&conf.api_endpoint, "test.local");
        assert_eq!(conf.zipcode, Some(8675309));
        assert_eq!(&conf.api_path, "weather/");
        Ok(())
    }
}
