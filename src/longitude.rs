use derive_more::{Display, FromStr, Into};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, hash::Hash};

use crate::{angle::Angle, format_string, Error};

/// Longitude in degrees, required be within the range -180.0 to 180.0
#[derive(
    Into, Clone, Copy, Display, FromStr, Debug, Serialize, Deserialize, PartialEq, Hash, Eq, Default,
)]
pub struct Longitude(Angle);

impl From<Longitude> for f64 {
    fn from(item: Longitude) -> Self {
        item.0.deg()
    }
}

impl TryFrom<f64> for Longitude {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if (-180.0..180.0).contains(&item) {
            Ok(Self(Angle::from_deg(item)))
        } else {
            Err(Error::InvalidValue(format_string!(
                "{item} is not a valid longitude"
            )))
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

    use crate::{longitude::Longitude, Error};

    #[test]
    fn test_longitude() -> Result<(), Error> {
        let h = Longitude::try_from(41.0)?;
        let v: f64 = h.into();
        assert_eq!(v, 41.0);

        let h1 = Longitude::try_from(41.0000)?;
        assert_eq!(h, h1);

        let mut hasher0 = DefaultHasher::new();
        h.hash(&mut hasher0);
        let mut hasher1 = DefaultHasher::new();
        h1.hash(&mut hasher1);
        assert_eq!(hasher0.finish(), hasher1.finish());

        let h = Longitude::try_from(-360.0);
        assert!(h.is_err());
        assert_eq!(
            h.err().unwrap().to_string(),
            format!("Invalid Value Error {} is not a valid longitude", -360.0)
        );
        Ok(())
    }

    #[test]
    fn test_longitude_not_eq_neg() -> Result<(), Error> {
        let a = Longitude::try_from(41.0)?;
        let b = Longitude::try_from(-41.0)?;
        assert_ne!(a, b);
        Ok(())
    }
}
