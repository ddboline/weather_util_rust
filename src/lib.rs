#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]

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

pub mod angle;
/// Configuration data
pub mod config;
/// Direction in degrees
pub mod direction;
/// Distance in meters
pub mod distance;
/// Relative Humidity in percent
pub mod humidity;
/// Latitude
pub mod latitude;
/// Longitude
pub mod longitude;
/// Precipitation (rain/snow) in mm
pub mod precipitation;
/// Pressure module: conversions between hPa, kPa, Pa
pub mod pressure;
/// Speed as meters per second
pub mod speed;
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

#[cfg(feature="stackstring")]
use stack_string::{SmallString, StackString};
#[cfg(feature="stackstring")]
pub type StringType = StackString;
#[cfg(feature="stackstring")]
pub type ApiStringType = SmallString<32>;

#[cfg(feature="stackstring")]
pub fn apistringtype_from_display(buf: impl std::fmt::Display) -> ApiStringType {
    SmallString::from_display(buf)
}

#[cfg(not(feature="stackstring"))]
pub type StringType = String;
#[cfg(not(feature="stackstring"))]
pub type ApiStringType = String;

#[cfg(not(feature="stackstring"))]
pub fn apistringtype_from_display(buf: impl std::fmt::Display) -> ApiStringType {
    format!("{buf}")
}

#[cfg(feature="stackstring")]
#[macro_export]
macro_rules! format_string {
    ($($arg:tt)*) => {
        {
            use std::fmt::Write;
            let mut buf = stack_string::StackString::new();
            std::write!(buf, "{}", std::format_args!($($arg)*)).unwrap();
            buf
        }
    };
}

#[cfg(not(feature="stackstring"))]
#[macro_export]
macro_rules! format_string {
    ($($arg:tt)*) => {
        {
            use std::fmt::Write;
            let mut buf = String::new();
            std::write!(buf, "{}", std::format_args!($($arg)*)).unwrap();
            buf
        }
    };
}
