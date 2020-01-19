use anyhow::{format_err, Error};
use derive_more::{Display, FromStr, Into};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

/// Latitude in degrees, required be within the range -180.0 to 180.0
#[derive(Into, Clone, Copy, Display, FromStr, Debug)]
pub struct Longitude(f64);

impl TryFrom<f64> for Longitude {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= -180.0 && item <= 180.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid latitude"))
        }
    }
}

impl Serialize for Longitude {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Longitude {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        f64::deserialize(deserializer).map(Self)
    }
}
