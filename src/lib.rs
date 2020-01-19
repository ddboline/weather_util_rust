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

/// Configuration data
pub mod config;
/// Latitude
pub mod latitude;
/// Longitude
pub mod longitude;
/// Temperature module: conversions between Kelvin, Ceclius and Fahrenheit
pub mod temperature;
/// Serialize/Deserialize Unix Timetstamp to/from DateTime
pub mod timestamp;
/// Representation of Weather Data from openweathermap.org
pub mod weather_data;
/// Representation of Weather Forecast from openweathermap.org
pub mod weather_forecast;
/// CLI App Options and implementation
pub mod weather_opts;
