use anyhow::{format_err, Error};
use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Relative Humidity as Percent
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display)]
#[serde(into = "i64", try_from = "i64")]
pub struct Humidity(i64);

impl TryFrom<i64> for Humidity {
    type Error = Error;
    fn try_from(item: i64) -> Result<Self, Self::Error> {
        if item >= 0 && item <= 100 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid relative humidity", item))
        }
    }
}
