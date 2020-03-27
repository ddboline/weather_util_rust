use anyhow::{format_err, Error};
use derive_more::{Add, Display, FromStr, Into};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    convert::{From, TryFrom},
    fmt::{self, Formatter},
    io::Write,
};

const MM_PER_INCH: f64 = 25.4;

/// Precipitation in mm
#[derive(Into, Clone, Copy, Display, FromStr, Debug, Serialize, Deserialize, Add, Default)]
#[serde(into = "f64", try_from = "f64")]
pub struct Precipitation(f64);

impl TryFrom<f64> for Precipitation {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item < 0.0 {
            Err(format_err!("{} is not a valid precipitation amount"))
        } else {
            Ok(Self(item))
        }
    }
}

impl Precipitation {
    pub fn from_mm(precip: f64) -> Result<Self, Error> {
        Self::try_from(precip)
    }

    pub fn from_inches(precip: f64) -> Result<Self, Error> {
        Self::try_from(precip * MM_PER_INCH)
    }

    pub fn millimeters(&self) -> f64 {
        self.0
    }

    pub fn inches(&self) -> f64 {
        self.0 / MM_PER_INCH
    }
}
