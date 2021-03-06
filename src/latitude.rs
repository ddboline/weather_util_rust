use anyhow::{format_err, Error};
use derive_more::{Display, FromStr, Into};
use rweb::Schema;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    convert::{From, TryFrom},
    fmt::{self, Formatter},
    hash::{Hash, Hasher},
    io::Write,
};

const HASH_FACTOR: f64 = 1_000_000.0;

/// Latitude in degrees, required be within the range -90.0 to 90.0
#[derive(Into, Clone, Copy, Display, FromStr, Debug, Serialize, Deserialize, Schema)]
#[serde(into = "f64", try_from = "f64")]
pub struct Latitude(f64);

impl PartialEq for Latitude {
    fn eq(&self, other: &Self) -> bool {
        (self.0 * HASH_FACTOR) as i32 == (other.0 * HASH_FACTOR) as i32
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
    use std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
    };

    use crate::latitude::{Latitude, HASH_FACTOR};

    #[test]
    fn test_latitude() -> Result<(), Error> {
        let h = Latitude::try_from(41.0)?;
        let v: f64 = h.into();
        assert_eq!(v, 41.0);

        let h1 = Latitude::try_from(41.0000)?;
        assert_eq!(h, h1);

        let mut hasher0 = DefaultHasher::new();
        h.hash(&mut hasher0);
        let mut hasher1 = DefaultHasher::new();
        h1.hash(&mut hasher1);
        assert_eq!(hasher0.finish(), hasher1.finish());

        let h = Latitude::try_from(-360.0);
        assert!(h.is_err());
        assert_eq!(
            h.err().unwrap().to_string(),
            format!("{} is not a valid latitude", -360.0)
        );
        Ok(())
    }

    #[test]
    fn test_latitude_not_eq_neg() -> Result<(), Error> {
        let a = Latitude::try_from(41.0)?;
        let b = Latitude::try_from(-41.0)?;
        assert_ne!(a, b);
        Ok(())
    }
}
