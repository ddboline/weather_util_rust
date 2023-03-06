use nutype::nutype;
use std::ops::Add;

use crate::{format_string, Error};

const MM_PER_INCH: f64 = 25.4;

/// Precipitation in mm
#[nutype(validate(min=0.0))]
#[derive(*, Serialize, Deserialize)]
pub struct Precipitation(f64);

impl Default for Precipitation {
    fn default() -> Self {
        Self::new(0.0).unwrap()
    }
}

impl Add for Precipitation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner().add(rhs.into_inner())).unwrap()
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
        Self::new(precip).map_err(|e| Error::InvalidValue(format_string!("{e}")))
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_inches(precip: f64) -> Result<Self, Error> {
        Self::new(precip * MM_PER_INCH).map_err(|e| Error::InvalidValue(format_string!("{e}")))
    }

    #[inline]
    #[must_use]
    pub fn millimeters(self) -> f64 {
        self.into_inner()
    }

    #[inline]
    #[must_use]
    pub fn inches(self) -> f64 {
        self.into_inner() / MM_PER_INCH
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use crate::{
        format_string,
        precipitation::{Precipitation, MM_PER_INCH},
        Error,
    };

    #[test]
    fn test_precipitation() -> Result<(), Error> {
        let p = Precipitation::new(1.0).map_err(|e| Error::InvalidValue(format_string!("{e}")))?;
        assert_eq!(p.millimeters(), 1.0);
        assert_eq!(p.inches(), 1.0 / MM_PER_INCH);
        let p2 = Precipitation::from_millimeters(1.0)?;
        assert_eq!(p, p2);
        let p = Precipitation::from_inches(1.0)?;
        assert_eq!(p.inches(), 1.0);

        let h = Precipitation::try_from(-1.0);
        assert!(h.is_err());
        Ok(())
    }
}
