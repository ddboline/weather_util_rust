use clap::Error as ClapError;
use envy::Error as EnvyError;
use serde_json::Error as SerdeJsonError;
use std::{fmt::Error as FmtError, io::Error as IoError, num::ParseFloatError};
use thiserror::Error;
use url::ParseError as UrlParseError;

#[cfg(feature = "cli")]
use reqwest::Error as ReqwestError;

use crate::StringType;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Value Error {0}")]
    InvalidValue(StringType),
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
    #[error("Invalid Input Error {0}")]
    InvalidInputError(StringType),
    #[error("Clap CLI Parser Error {0}")]
    ClapError(#[from] ClapError),

    #[cfg(feature = "cli")]
    #[error("Reqwest Error {0}")]
    ReqwestError(#[from] ReqwestError),
}
