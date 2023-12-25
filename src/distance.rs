use nutype::nutype;
use std::convert::TryFrom;

use crate::Error;

const METERS_PER_MILE: f64 = 1609.344;

/// Distance in meters
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
pub struct Distance(f64);

impl Default for Distance {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl Distance {
    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_meters(meters: f64) -> Result<Self, Error> {
        Self::try_from(meters).map_err(Into::into)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_miles(miles: f64) -> Result<Self, Error> {
        Self::try_from(miles * METERS_PER_MILE).map_err(Into::into)
    }

    #[must_use]
    pub fn meters(self) -> f64 {
        self.into_inner()
    }

    #[must_use]
    pub fn miles(self) -> f64 {
        self.meters() / METERS_PER_MILE
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
        assert_eq!(
            &format!("{s:?}"),
            "Err(DistanceError(GreaterOrEqualViolated))"
        );
        Ok(())
    }
}
