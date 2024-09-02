use nutype::nutype;

use crate::{format_string, Error};

const FREEZING_POINT_KELVIN: f64 = 273.15;
const FAHRENHEIT_OFFSET: f64 = 459.67;
const FAHRENHEIT_FACTOR: f64 = 1.8;

/// Temperature struct, data is stored as Kelvin
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
        Into,
        PartialOrd,
    )
)]
pub struct Temperature(f64);

impl Default for Temperature {
    fn default() -> Self {
        Self::try_new(0.0).unwrap()
    }
}

impl Temperature {
    /// ```
    /// use weather_util_rust::temperature::Temperature;
    /// # use anyhow::Error;
    /// # fn main() -> Result<(), Error> {
    /// let temp = Temperature::from_celcius(0.0)?;
    /// assert_eq!(temp.kelvin(), 273.15);
    /// assert!((temp.fahrenheit() - 32.0).abs() < 1e-6);
    /// assert!((temp.celcius() - 0.0).abs() < 1e-6);
    /// # Ok(())
    /// # }
    /// ```

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_kelvin(t: f64) -> Result<Self, Error> {
        Self::try_new(t).map_err(|e| {
            Error::InvalidValue(format_string!(
                "{e}: {t} is not a valid temperature in Kelvin"
            ))
        })
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_celcius(t: f64) -> Result<Self, Error> {
        Self::try_new(t + FREEZING_POINT_KELVIN).map_err(|e| {
            Error::InvalidValue(format_string!(
                "{e}: {t} is not a valid temperature in Celcius"
            ))
        })
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_fahrenheit(t: f64) -> Result<Self, Error> {
        Self::try_new((t + FAHRENHEIT_OFFSET) / FAHRENHEIT_FACTOR).map_err(|e| {
            Error::InvalidValue(format_string!(
                "{e}: {t} is not a valid temperature in Fahrenheit",
            ))
        })
    }

    #[inline]
    #[must_use]
    pub fn kelvin(self) -> f64 {
        self.into_inner()
    }

    #[inline]
    #[must_use]
    pub fn celcius(self) -> f64 {
        self.into_inner() - FREEZING_POINT_KELVIN
    }

    #[inline]
    #[must_use]
    pub fn fahrenheit(self) -> f64 {
        self.into_inner() * FAHRENHEIT_FACTOR - FAHRENHEIT_OFFSET
    }
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use std::convert::TryFrom;

    use crate::{format_string, temperature::Temperature, Error};

    #[test]
    fn test_temperature() -> Result<(), Error> {
        let t = Temperature::from_celcius(15.0)?;
        assert_eq!(t.celcius(), 15.0);

        let t = Temperature::from_fahrenheit(15.0)?;
        assert_abs_diff_eq!(t.fahrenheit(), 15.0, epsilon = 0.0001);

        let t = Temperature::try_new(300.0).map_err(|e| Error::InvalidValue(format_string!("{e}")))?;
        assert_abs_diff_eq!(t.kelvin(), 300.0);

        let t = Temperature::try_from(-15.0);
        assert!(t.is_err());

        let t = Temperature::from_celcius(-300.0);
        assert!(t.is_err());
        assert_eq!(
            t.err().unwrap().to_string(),
            format!(
                "Invalid Value Error Temperature is too small. The value must be greater or equal \
                 to 0.0.: {} is not a valid temperature in Celcius",
                -300.0
            )
        );

        let t = Temperature::from_fahrenheit(-500.0);
        assert!(t.is_err());
        assert_eq!(
            t.err().unwrap().to_string(),
            format!(
                "Invalid Value Error Temperature is too small. The value must be greater or equal \
                 to 0.0.: {} is not a valid temperature in Fahrenheit",
                -500.0
            )
        );
        Ok(())
    }
}
