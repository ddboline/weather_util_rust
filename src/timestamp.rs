use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

use crate::datetime_wrapper::DateTimeWrapper;

/// ! serialize function required by `#[serde(with=timestamp)]`
pub fn serialize<S>(date: &DateTimeWrapper, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.timestamp())
}

/// ! deserialize function required by `#[serde(with=timestamp)]`
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTimeWrapper, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer).map(|t| Utc.timestamp(t, 0).into())
}
