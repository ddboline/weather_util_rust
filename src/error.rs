use envy::Error as EnvyError;
use serde_json::Error as SerdeJsonError;
use std::{fmt::Error as FmtError, io::Error as IoError, num::ParseFloatError};
use thiserror::Error;
use url::ParseError as UrlParseError;

use crate::{
    distance::DistanceError, humidity::HumidityError, precipitation::PrecipitationError,
    pressure::PressureError, speed::SpeedError, temperature::TemperatureError,
    timezone::TimeZoneError,
};

#[cfg(feature = "cli")]
use clap::Error as ClapError;

#[cfg(feature = "cli")]
use reqwest::Error as ReqwestError;

use crate::StringType;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Format Error {0}")]
    FmtError(#[from] FmtError),
    #[error("Float Parse Error {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Environment Parsing Error {0}")]
    EnvyError(#[from] EnvyError),
    #[error("URL Parse Error {0}")]
    UrlParseError(#[from] UrlParseError),
    #[error("JSON Serde Error {0}")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("IO Error {0}")]
    IoError(#[from] IoError),
    #[error("Invalid Value Error {0}")]
    InvalidValue(StringType),
    #[error("Invalid Input Error {0}")]
    InvalidInputError(StringType),
    #[error("DistanceError {0}")]
    DistanceError(#[from] DistanceError),
    #[error("HumidityError {0}")]
    HumidityError(#[from] HumidityError),
    #[error("Invalid Latitude")]
    InvalidLatitude,
    #[error("Invalid Longitude")]
    InvalidLongitude,
    #[error("PrecipitationError {0}")]
    PrecipitationError(#[from] PrecipitationError),
    #[error("PressureError {0}")]
    PressureError(#[from] PressureError),
    #[error("SpeedError {0}")]
    SpeedError(#[from] SpeedError),
    #[error("TemperatureError {0}")]
    TemperatureError(#[from] TemperatureError),
    #[error("TimeZoneError {0}")]
    TimeZoneError(#[from] TimeZoneError),

    #[cfg(feature = "cli")]
    #[error("Clap CLI Parser Error {0}")]
    ClapError(#[from] ClapError),

    #[cfg(feature = "cli")]
    #[error("Reqwest Error {0}")]
    ReqwestError(#[from] ReqwestError),
}
