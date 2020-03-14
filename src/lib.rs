#![allow(unused_imports)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::similar_names)]
#![allow(clippy::shadow_unrelated)]
#![allow(clippy::missing_errors_doc)]

//! Utility to retreive and format weather data from openweathermap.org
//!
//! ```bash
//! Please specify one of zipcode(country_code), city_name, or lat and lon.
//!
//! USAGE:
//! weather-util-rust [OPTIONS]
//!
//! FLAGS:
//! -h, --help       Prints help information
//! -V, --version    Prints version information
//!
//! OPTIONS:
//! -k, --api-key <api-key>              Api key (optional but either this or API_KEY environemnt variable must exist)
//!     --city-name <city-name>          City Name (optional)
//! -c, --country-code <country-code>    Country Code (optional), if not specified `us` will be assumed
//!     --lat <lat>                      Latitude (must also specify Longitude)
//!     --lon <lon>                      Longitude (must also specify Latitude)
//! -z, --zipcode <zipcode>              Zipcode (optional)
//!

/// Configuration data
pub mod config;
/// Latitude
pub mod latitude;
/// Longitude
pub mod longitude;
/// Pressure module: conversions between hPa, kPa, Pa
pub mod pressure;
/// Relative Humidity in percent
pub mod humidity;
/// Speed as meters per second
pub mod speed;
/// Distance in meters
pub mod distance;
/// Direction in degrees
pub mod direction;
/// Temperature module: conversions between Kelvin, Ceclius and Fahrenheit
pub mod temperature;
/// Serialize/Deserialize Unix Timetstamp to/from `DateTime`
pub mod timestamp;
/// Timezone offset as seconds before / after UTC
pub mod timezone;
/// Reqwest Client
pub mod weather_api;
/// Representation of Weather Data from openweathermap.org
pub mod weather_data;
/// Representation of Weather Forecast from openweathermap.org
pub mod weather_forecast;
/// CLI App Options and implementation
pub mod weather_opts;

use anyhow::{format_err, Error};
use log::error;
use retry::{
    delay::{jitter, Exponential},
    retry,
};
