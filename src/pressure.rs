use anyhow::{format_err, Error};
use derive_more::Into;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use rweb::Schema;

const HECTO: f64 = 1.0; // hPa 100 hundred Pa
const KILO: f64 = 1_000.0 / 100.0;
const ATM: f64 = 98.0665 * HECTO / KILO;
const PSI: f64 = 14.223 / (98.0665 * HECTO / KILO);

/// Pressure struct, data is stored as hPa (100 Pa)
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Schema)]
#[serde(into = "f64", try_from = "f64")]
pub struct Pressure(f64);

impl TryFrom<f64> for Pressure {
    type Error = Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        if item > 0.0 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid pressure value", item))
        }
    }
}

impl Pressure {
    pub fn from_kpa(kpa: f64) -> Result<Self, Error> {
        Self::try_from(kpa * HECTO / KILO)
    }

    pub fn from_hpa(hpa: f64) -> Result<Self, Error> {
        Self::try_from(hpa)
    }

    pub fn from_atmosphere(atm: f64) -> Result<Self, Error> {
        Self::try_from(atm * ATM)
    }

    pub fn from_atm(atm: f64) -> Result<Self, Error> {
        Self::from_atmosphere(atm)
    }

    pub fn from_psi(psi: f64) -> Result<Self, Error> {
        Self::try_from(psi / PSI)
    }

    pub fn kpa(self) -> f64 {
        self.0 * KILO / HECTO
    }

    pub fn hpa(self) -> f64 {
        self.0
    }

    pub fn atmosphere(self) -> f64 {
        self.0 / ATM
    }

    pub fn atm(self) -> f64 {
        self.atmosphere()
    }

    pub fn psi(self) -> f64 {
        self.0 * PSI
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use approx::assert_abs_diff_eq;

    use crate::pressure::Pressure;

    #[test]
    fn test_pressure() -> Result<(), Error> {
        let p = Pressure::from_atmosphere(1.0)?;
        assert_eq!(p.atm(), 1.0);
        assert_abs_diff_eq!(p.hpa(), 98.0665 / 10.0, epsilon = 0.00001);
        assert_abs_diff_eq!(p.kpa(), 98.0665, epsilon = 0.00001);
        assert_abs_diff_eq!(p.psi(), 14.223, epsilon = 0.00001);
        let p = Pressure::from_kpa(1.0)?;
        assert_abs_diff_eq!(p.kpa(), 1.0);
        let p = Pressure::from_atmosphere(1.0)?;
        let p2 = Pressure::from_atm(1.0)?;
        assert_eq!(p, p2);
        let p = Pressure::from_hpa(1.0)?;
        assert_eq!(p.hpa(), 1.0);

        let p = Pressure::from_hpa(-1.0);
        assert!(p.is_err());
        assert_eq!(
            p.err().unwrap().to_string(),
            format!("{} is not a valid pressure value", -1.0)
        );
        Ok(())
    }
}
