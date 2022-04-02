use serde::{self, de::Error, Deserialize, Deserializer, Serializer};
use time::OffsetDateTime;

/// ! serialize function required by `#[serde(with=timestamp)]`
/// # Errors
///
/// Will return error if serialization fails
pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.unix_timestamp())
}

/// ! deserialize function required by `#[serde(with=timestamp)]`
/// # Errors
///
/// Will return error if deserialization fails
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer).and_then(|t| {
        OffsetDateTime::from_unix_timestamp(t).map_err(|e| D::Error::custom(format!("{:?}", e)))
    })
}
