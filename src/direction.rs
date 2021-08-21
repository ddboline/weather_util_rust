use anyhow::{format_err, Error};
use derive_more::{Display, Into, Deref, From};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, f64::consts::PI};
use rust_decimal::Decimal;

use crate::angle::Angle;

/// Direction in degrees
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display, Deref, From)]
pub struct Direction(Angle);

impl From<f64> for Direction {
    fn from(item: f64) -> Self {
        Self(Angle::from_deg(item))
    }
}

impl Direction {
    pub fn from_deg(deg: f64) -> Self {
        Self(Angle::from_deg(deg))
    }

    pub fn from_radian(rad: f64) -> Self {
        Self(Angle::from_radian(rad))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    use crate::direction::Direction;

    #[test]
    fn test_direction() {
        assert_abs_diff_eq!(
            Direction::from_deg(90.).deg(),
            Direction::from_deg(90. + 360.).deg()
        );
        assert_abs_diff_eq!(
            Direction::from_deg(90.).deg(),
            Direction::from_radian(PI / 2.).deg()
        );
        assert_abs_diff_eq!(
            Direction::from_deg(90.).deg(),
            Direction::from_radian(PI / 2. + 2. * PI).deg()
        );
        assert_abs_diff_eq!(
            Direction::from_deg(90.).radian(),
            Direction::from_radian(PI / 2.).radian()
        );

        assert_abs_diff_eq!(
            Direction::from_deg(-90.).deg(),
            Direction::from_deg(-90. + 360.).deg()
        );
        assert_abs_diff_eq!(
            Direction::from_deg(-90.).deg(),
            Direction::from_radian(-1.0 * PI / 2.).deg()
        );
        assert_abs_diff_eq!(
            Direction::from_deg(-90.).deg(),
            Direction::from_radian(-1.0 * PI / 2. + 2. * PI).deg()
        );
        assert_abs_diff_eq!(
            Direction::from_deg(-90.).radian(),
            Direction::from_radian(-1.0 * PI / 2.).radian()
        );
    }
}
