use chrono::{DateTime, Utc};
use derive_more::{Deref, From, FromStr, Into};
use rweb::openapi::{Entity, Schema, Type};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    FromStr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    Into,
    From,
)]
pub struct DateTimeWrapper(DateTime<Utc>);

impl Entity for DateTimeWrapper {
    #[inline]
    fn describe() -> Schema {
        Schema {
            schema_type: Some(Type::String),
            format: "datetime".into(),
            ..Schema::default()
        }
    }
}
