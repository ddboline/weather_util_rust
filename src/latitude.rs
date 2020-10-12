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
use std::hash::{Hash, Hasher};

const HASH_FACTOR: f64 = 1_000_000.0;

/// Latitude in degrees, required be within the range -90.0 to 90.0
#[derive(Into, Clone, Copy, Display, FromStr, Debug, Serialize, Deserialize)]
#[serde(into = "f64", try_from = "f64")]
pub struct Latitude(f64);

impl PartialEq for Latitude {
    fn eq(&self, other: &Self) -> bool {
        (self.0 * HASH_FACTOR) as u32 == (other.0 * HASH_FACTOR) as u32
        }
}

impl Hash for Latitude {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.0 * HASH_FACTOR) as u32).hash(state);
    }
}


impl TryFrom<f64> for Latitude {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= -90.0 && item <= 90.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid latitude", item))
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use std::convert::TryFrom;

    use crate::latitude::Latitude;

    #[test]
    fn test_latitude() -> Result<(), Error> {
        let h = Latitude::try_from(41.0)?;
        let v: f64 = h.into();
        assert_eq!(v, 41.0);

        let h = Latitude::try_from(-360.0);
        assert!(h.is_err());
        assert_eq!(
            h.err().unwrap().to_string(),
            format!("{} is not a valid latitude", -360.0)
        );
        Ok(())
    }
}
