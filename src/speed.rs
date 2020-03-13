use anyhow::{format_err, Error};
use derive_more::Into;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

const SECONDS_PER_HOUR: f64 = 3600.;
const METERS_PER_MILE: f64 = 1609.344;

/// Speed in meters per second
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize)]
pub struct Speed(f64);

impl TryFrom<f64> for Speed {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= 0.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid speed", item))
        }
    }
}

impl Speed {
    pub fn from_mps(mps: f64) -> Result<Self, Error> {
        Self::try_from(mps)
    }

    pub fn from_mph(mph: f64) -> Result<Self, Error> {
        Self::try_from(mph * METERS_PER_MILE / SECONDS_PER_HOUR)
    }

    pub fn mps(&self) -> f64 {
        self.0
    }

    pub fn mph(&self) -> f64 {
        self.0 * SECONDS_PER_HOUR / METERS_PER_MILE
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use anyhow::Error;

    use crate::speed::Speed;

    #[test]
    fn test_speed() -> Result<(), Error> {
        let s = Speed::from_mph(1.0)?;
        assert_abs_diff_eq!(s.mph(), 1.0);
        assert_abs_diff_eq!(s.mps(), 1609.344 / 3600.);
        Ok(())
    }
}
