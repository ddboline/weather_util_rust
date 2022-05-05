use derive_more::Into;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{format_string, Error};

const METERS_PER_MILE: f64 = 1609.344;

/// Distance in meters
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Default)]
#[serde(into = "f64", try_from = "f64")]
pub struct Distance(f64);

impl TryFrom<f64> for Distance {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= 0.0 {
            Ok(Self(item))
        } else {
            Err(Error::InvalidValue(format_string!(
                "{item} is not a valid distance"
            )))
        }
    }
}

impl Distance {
    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_meters(meters: f64) -> Result<Self, Error> {
        Self::try_from(meters)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_miles(miles: f64) -> Result<Self, Error> {
        Self::try_from(miles * METERS_PER_MILE)
    }

    #[must_use]
    pub fn meters(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn miles(self) -> f64 {
        self.0 / METERS_PER_MILE
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{distance::Distance, Error};

    #[test]
    fn test_distance() -> Result<(), Error> {
        let s = Distance::from_miles(1.0)?;
        assert_abs_diff_eq!(s.miles(), 1.0);
        assert_abs_diff_eq!(s.meters(), 1609.344);

        let s = Distance::from_meters(160934.4)?;
        assert_abs_diff_eq!(s.miles(), 100.0);
        assert_abs_diff_eq!(s.meters(), 160934.4);
        Ok(())
    }

    #[test]
    fn test_invalid_distance() -> Result<(), Error> {
        let s = Distance::from_miles(-12.0);
        assert!(s.is_err());
        assert_eq!(
            s.err().unwrap().to_string(),
            format!("Invalid Value Error {} is not a valid distance", -19312.128)
        );
        Ok(())
    }
}
