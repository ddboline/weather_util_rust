use anyhow::{format_err, Error};
use chrono::offset::FixedOffset;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Direction in degrees
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display)]
#[serde(into = "i32", try_from = "i32")]
pub struct TimeZone(i32);

impl TryFrom<i32> for TimeZone {
    type Error = Error;
    fn try_from(item: i32) -> Result<Self, Self::Error> {
        if item > -86400 && item < 86400 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid timezone"))
        }
    }
}

impl std::convert::Into<FixedOffset> for TimeZone {
    fn into(self) -> FixedOffset {
        FixedOffset::east(self.0)
    }
}
