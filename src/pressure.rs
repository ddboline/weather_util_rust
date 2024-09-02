use nutype::nutype;

use crate::Error;

const HECTO: f64 = 1.0; // hPa 100 hundred Pa
const KILO: f64 = 1_000.0 / 100.0;
const ATM: f64 = 98.0665 * HECTO / KILO;
const PSI: f64 = 14.223 / (98.0665 * HECTO / KILO);

/// Pressure struct, data is stored as hPa (100 Pa)
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
pub struct Pressure(f64);

impl Default for Pressure {
    fn default() -> Self {
        Self::try_new(0.0).unwrap()
    }
}

impl Pressure {
    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_kpa(kpa: f64) -> Result<Self, Error> {
        Self::try_new(kpa * HECTO / KILO).map_err(Into::into)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_hpa(hpa: f64) -> Result<Self, Error> {
        Self::try_new(hpa).map_err(Into::into)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_atmosphere(atm: f64) -> Result<Self, Error> {
        Self::try_new(atm * ATM).map_err(Into::into)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_atm(atm: f64) -> Result<Self, Error> {
        Self::from_atmosphere(atm)
    }

    /// # Errors
    ///
    /// Will return error if input is less than zero
    pub fn from_psi(psi: f64) -> Result<Self, Error> {
        Self::try_new(psi / PSI).map_err(Into::into)
    }

    #[inline]
    #[must_use]
    pub fn kpa(self) -> f64 {
        self.into_inner() * KILO / HECTO
    }

    #[inline]
    #[must_use]
    pub fn hpa(self) -> f64 {
        self.into_inner()
    }

    #[inline]
    #[must_use]
    pub fn atmosphere(self) -> f64 {
        self.into_inner() / ATM
    }

    #[inline]
    #[must_use]
    pub fn atm(self) -> f64 {
        self.atmosphere()
    }

    #[inline]
    #[must_use]
    pub fn psi(self) -> f64 {
        self.into_inner() * PSI
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{pressure::Pressure, Error};

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
        assert_eq!(
            &format!("{p:?}"),
            "Err(PressureError(GreaterOrEqualViolated))"
        );
        Ok(())
    }
}
