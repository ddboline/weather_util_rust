use anyhow::{format_err, Error};
use derive_more::{Into, From};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

const FREEZING_POINT_KELVIN: f64 = 273.15;
const FAHRENHEIT_OFFSET: f64 = 459.67;
const FAHRENHEIT_FACTOR: f64 = 1.8;

/// Temperature struct, data is stored as Kelvin
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize)]
#[serde(into = "f64", try_from = "f64")]
pub struct Temperature(f64);

impl TryFrom<f64> for Temperature {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item >= 0.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid Temperature", item))
        }
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
    pub fn from_celcius(t: f64) -> Result<Self, Error> {
        if t >= -FREEZING_POINT_KELVIN {
            Ok(Self(t + FREEZING_POINT_KELVIN))
        } else {
            Err(format_err!("{} is not a valid temperature in Celcius"))
        }
    }
    pub fn from_fahrenheit(t: f64) -> Result<Self, Error> {
        if t >= -FAHRENHEIT_OFFSET {
            Ok(Self((t + FAHRENHEIT_OFFSET) / FAHRENHEIT_FACTOR))
        } else {
            Err(format_err!("{} is not a valid temperature in Fahrenheit"))
        }
    }
    pub fn kelvin(self) -> f64 {
        self.0
    }
    pub fn celcius(self) -> f64 {
        self.0 - FREEZING_POINT_KELVIN
    }
    pub fn fahrenheit(self) -> f64 {
        self.0 * FAHRENHEIT_FACTOR - FAHRENHEIT_OFFSET
    }
}
