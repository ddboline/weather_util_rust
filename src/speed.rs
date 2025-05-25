use nutype::nutype;

use crate::Error;

const SECONDS_PER_HOUR: f64 = 3600.;
const METERS_PER_MILE: f64 = 1609.344;

/// Speed in meters per second
#[nutype(
    validate(greater_or_equal = 0.0),
    derive(
        Display,
        TryFrom,
        AsRef,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        PartialEq,
        Debug,
        Into
    )
)]
pub struct Speed(f64);

impl Default for Speed {
    fn default() -> Self {
        Self::try_new(0.0).unwrap()
    }
}

impl Speed {
    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_mps(mps: f64) -> Result<Self, Error> {
        Self::try_new(mps).map_err(Into::into)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_mph(mph: f64) -> Result<Self, Error> {
        Self::try_new(mph * METERS_PER_MILE / SECONDS_PER_HOUR).map_err(Into::into)
    }

    #[inline]
    #[must_use]
    pub fn mps(self) -> f64 {
        self.into_inner()
    }

    #[inline]
    #[must_use]
    pub fn mph(self) -> f64 {
        self.into_inner() * SECONDS_PER_HOUR / METERS_PER_MILE
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{Error, speed::Speed};

    #[test]
    fn test_speed() -> Result<(), Error> {
        let s = Speed::from_mph(1.0)?;
        assert_abs_diff_eq!(s.mph(), 1.0);
        assert_abs_diff_eq!(s.mps(), 1609.344 / 3600.);

        let s = Speed::from_mps(1.0)?;
        assert_abs_diff_eq!(s.mps(), 1.0);

        let s = Speed::from_mps(-1.0);
        assert_eq!(&format!("{s:?}"), "Err(SpeedError(GreaterOrEqualViolated))");
        Ok(())
    }
}
