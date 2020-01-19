use anyhow::{format_err, Error};
use std::env::var;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

/// Configuration data
#[derive(Default, Debug)]
pub struct ConfigInner {
    /// openweathermap.org api key
    pub api_key: String,
    /// openweathermap.org api endpoint
    pub api_endpoint: String,
}

/// Configuration struct
#[derive(Default, Debug, Clone)]
pub struct Config(Arc<ConfigInner>);

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pull in configuration data using `[dotenv](https://crates.io/dotenv)`.
    /// If a .env file exists in the current directory, pull in any ENV variables in it.
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
    /// assert_eq!(config.api_key, var("API_KEY")?);
    /// assert_eq!(config.api_endpoint, var("API_ENDPOINT")?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn init_config() -> Result<Self, Error> {
        let fname = "config.env";
        let home_dir = var("HOME").map_err(|e| format_err!("No HOME variable {}", e))?;
        let default_fname = format!("{}/.config/weather_util/config.env", home_dir);
        let env_file = if Path::new(fname).exists() {
            fname.to_string()
        } else {
            default_fname
        };

        dotenv::dotenv().ok();

        if Path::new(&env_file).exists() {
            dotenv::from_path(&env_file).ok();
        } else if Path::new("config.env").exists() {
            dotenv::from_filename("config.env").ok();
        }

        let inner = ConfigInner {
            api_key: var("API_KEY")?,
            api_endpoint: var("API_ENDPOINT")?,
        };

        Ok(Self(Arc::new(inner)))
    }
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
