use derive_more::From;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

const FREEZING_POINT_KELVIN: f64 = 273.15;
const FAHRENHEIT_OFFSET: f64 = 459.67;
const FAHRENHEIT_FACTOR: f64 = 1.8;

#[derive(Debug, From, PartialEq, Copy, Clone, PartialOrd)]
pub struct Temperature(f64);

impl Temperature {
    pub fn from_celcius(t: f64) -> Self {
        assert!(t >= -FREEZING_POINT_KELVIN);
        Self(t + FREEZING_POINT_KELVIN)
    }
    pub fn from_fahrenheit(t: f64) -> Self {
        assert!(t >= -FAHRENHEIT_OFFSET);
        Self((t + FAHRENHEIT_OFFSET) / FAHRENHEIT_FACTOR)
    }
    pub fn celcius(&self) -> f64 {
        self.0 - FREEZING_POINT_KELVIN
    }
    pub fn celc(&self) -> f64 {
        self.celcius()
    }
    pub fn fahrenheit(&self) -> f64 {
        self.0 * FAHRENHEIT_FACTOR - FAHRENHEIT_OFFSET
    }
    pub fn fahr(&self) -> f64 {
        self.fahrenheit()
    }
}

impl Serialize for Temperature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Temperature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        f64::deserialize(deserializer).map(Temperature)
    }
}
