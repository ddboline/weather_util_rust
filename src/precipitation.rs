use derive_more::{Add, Display, FromStr, Into};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{format_string, Error};

const MM_PER_INCH: f64 = 25.4;

/// Precipitation in mm
#[derive(
    Into,
    Clone,
    Copy,
    Display,
    FromStr,
    Debug,
    Serialize,
    Deserialize,
    Add,
    Default,
    PartialEq,
    PartialOrd,
)]
#[serde(into = "f64", try_from = "f64")]
pub struct Precipitation(f64);

impl TryFrom<f64> for Precipitation {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item < 0.0 {
            Err(Error::InvalidValue(format_string!(
                "{item} is not a valid precipitation amount"
            )))
        } else {
            Ok(Self(item))
        }
    }
}

impl Precipitation {
    /// ```
    /// use weather_util_rust::precipitation::Precipitation;
    /// # use anyhow::Error;
    /// # fn main() -> Result<(), Error> {
    /// let rain = Precipitation::from_inches(1.0)?;
    /// assert_eq!(rain.millimeters(), 25.4);
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_millimeters(precip: f64) -> Result<Self, Error> {
        Self::try_from(precip)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_inches(precip: f64) -> Result<Self, Error> {
        Self::try_from(precip * MM_PER_INCH)
    }

    #[inline]
    #[must_use]
    pub fn millimeters(self) -> f64 {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn inches(self) -> f64 {
        self.0 / MM_PER_INCH
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use crate::{
        precipitation::{Precipitation, MM_PER_INCH},
        Error,
    };

    #[test]
    fn test_precipitation() -> Result<(), Error> {
        let p = Precipitation::try_from(1.0)?;
        assert_eq!(p.millimeters(), 1.0);
        assert_eq!(p.inches(), 1.0 / MM_PER_INCH);
        let p2 = Precipitation::from_millimeters(1.0)?;
        assert_eq!(p, p2);
        let p = Precipitation::from_inches(1.0)?;
        assert_eq!(p.inches(), 1.0);

        let h = Precipitation::try_from(-1.0);
        assert!(h.is_err());
        assert_eq!(
            h.err().unwrap().to_string(),
            format!(
                "Invalid Value Error {} is not a valid precipitation amount",
                -1.0
            )
        );
        Ok(())
    }
}
