use anyhow::{format_err, Error};
use derive_more::{Add, Display, FromStr, Into};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    convert::{From, TryFrom},
    fmt::{self, Formatter},
    io::Write,
};
use rweb::Schema;

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
    Schema,
)]
#[serde(into = "f64", try_from = "f64")]
pub struct Precipitation(f64);

impl TryFrom<f64> for Precipitation {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item < 0.0 {
            Err(format_err!("{} is not a valid precipitation amount", item))
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
    pub fn from_millimeters(precip: f64) -> Result<Self, Error> {
        Self::try_from(precip)
    }

    pub fn from_inches(precip: f64) -> Result<Self, Error> {
        Self::try_from(precip * MM_PER_INCH)
    }

    #[inline]
    pub fn millimeters(self) -> f64 {
        self.0
    }

    #[inline]
    pub fn inches(self) -> f64 {
        self.0 / MM_PER_INCH
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use std::convert::TryFrom;

    use crate::precipitation::{Precipitation, MM_PER_INCH};

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
            format!("{} is not a valid precipitation amount", -1.0)
        );
        Ok(())
    }
}
