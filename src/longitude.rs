use anyhow::{format_err, Error};
use derive_more::{Display, FromStr, Into};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

const HASH_FACTOR: f64 = 1_000_000.0;

/// Latitude in degrees, required be within the range -180.0 to 180.0
#[derive(Into, Clone, Copy, Display, FromStr, Debug, Serialize, Deserialize)]
#[serde(into = "f64", try_from = "f64")]
pub struct Longitude(f64);

impl PartialEq for Longitude {
    fn eq(&self, other: &Self) -> bool {
        (self.0 * HASH_FACTOR) as u32 == (other.0 * HASH_FACTOR) as u32
        }
}

impl Hash for Longitude {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.0 * HASH_FACTOR) as u32).hash(state);
    }
}

impl TryFrom<f64> for Longitude {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= -180.0 && item <= 180.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid longitude", item))
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use std::convert::TryFrom;

    use crate::longitude::Longitude;

    #[test]
    fn test_longitude() -> Result<(), Error> {
        let h = Longitude::try_from(41.0)?;
        let v: f64 = h.into();
        assert_eq!(v, 41.0);

        let h = Longitude::try_from(-360.0);
        assert!(h.is_err());
        assert_eq!(
            h.err().unwrap().to_string(),
            format!("{} is not a valid longitude", -360.0)
        );
        Ok(())
    }
}
