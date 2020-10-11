use anyhow::{format_err, Error};
use derive_more::{Display, FromStr, Into};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    convert::{From, TryFrom},
    fmt::{self, Formatter},
    io::Write,
};

/// Latitude in degrees, required be within the range -90.0 to 90.0
#[derive(Into, Clone, Copy, Display, FromStr, Debug, Serialize, Deserialize, PartialEq)]
#[serde(into = "f64", try_from = "f64")]
pub struct Latitude(f64);

impl TryFrom<f64> for Latitude {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= -90.0 && item <= 90.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid latitude"))
        }
    }
}
