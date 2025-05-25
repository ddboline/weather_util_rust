use derive_more::{Display, FromStr, Into};
use serde::{Deserialize, Serialize};
use std::{
    convert::{From, TryFrom},
    hash::Hash,
};

use crate::{Error, angle::Angle};

/// Latitude in degrees, required be within the range -90.0 to 90.0
#[derive(
    Into,
    derive_more::From,
    Clone,
    Copy,
    Display,
    FromStr,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Default,
)]
pub struct Latitude(Angle);

impl From<Latitude> for f64 {
    fn from(item: Latitude) -> Self {
        item.0.deg()
    }
}

impl TryFrom<f64> for Latitude {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if (-90.0..90.0).contains(&item) {
            Ok(Angle::from_deg(item).into())
        } else {
            Err(Error::InvalidLatitude)
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
    };

    use crate::{Error, latitude::Latitude};

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
        assert_eq!(&h.err().unwrap().to_string(), "Invalid Latitude",);
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
