use anyhow::Error;
use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, f64::consts::PI, fmt, str::FromStr};

const RADIANS_PER_TURN: f64 = 2.0 * PI;

/// Angle in arcseconds
#[derive(Into, Debug, Copy, Clone, PartialOrd, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(into = "f64", from = "f64")]
pub struct Angle {
    degree: i16,
    minute: u8,
    second: u8,
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0.5}", self.deg())
    }
}

impl FromStr for Angle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deg: f64 = s.parse()?;
        Ok(Self::from_deg(deg))
    }
}

impl From<f64> for Angle {
    fn from(item: f64) -> Self {
        Self::from_deg(item)
    }
}

impl From<Angle> for f64 {
    fn from(item: Angle) -> Self {
        item.deg()
    }
}

impl Angle {
    pub fn from_deg_min_sec(degree: i16, minute: u8, second: u8) -> Self {
        Self {
            degree,
            minute,
            second,
        }
    }

    pub fn from_deg(deg: f64) -> Self {
        let degree = deg as i64;
        let minute = (deg * 60.0) as i64 - (degree * 60);
        let second = (deg * 3600.0) as i64 - minute * 60 - degree * 3600;
        let degree = if degree >= 0 {
            degree % 360
        } else {
            (degree % 360) + 360
        } as i16;
        let minute = if minute >= 0 {
            minute % 60
        } else {
            (minute % 60) + 60
        } as u8;
        let second = if second >= 0 {
            second % 60
        } else {
            (second % 60) + 60
        } as u8;
        Self {
            degree,
            minute,
            second,
        }
    }

    pub fn from_radian(rad: f64) -> Self {
        Self::from(rad * 360.0 / RADIANS_PER_TURN)
    }

    #[inline]
    pub fn deg(self) -> f64 {
        self.degree as f64 + (self.minute as f64 / 60.0) + (self.second as f64 / 3600.0)
    }

    pub fn deg_min_sec(self) -> (i16, u8, u8) {
        (self.degree, self.minute, self.second)
    }

    #[inline]
    pub fn radian(self) -> f64 {
        self.deg() * RADIANS_PER_TURN / 360.0
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    use crate::angle::Angle;

    #[test]
    fn test_direction() {
        assert_abs_diff_eq!(
            Angle::from_deg(90.).deg(),
            Angle::from_deg(90. + 360.).deg()
        );
        assert_abs_diff_eq!(
            Angle::from_deg(90.).deg(),
            Angle::from_radian(PI / 2.).deg()
        );
        assert_abs_diff_eq!(
            Angle::from_deg(90.).deg(),
            Angle::from_radian(PI / 2. + 2. * PI).deg()
        );
        assert_abs_diff_eq!(
            Angle::from_deg(90.).radian(),
            Angle::from_radian(PI / 2.).radian()
        );

        assert_abs_diff_eq!(
            Angle::from_deg(-90.).deg(),
            Angle::from_deg(-90. + 360.).deg()
        );
        assert_abs_diff_eq!(
            Angle::from_deg(-90.).deg(),
            Angle::from_radian(-1.0 * PI / 2.).deg()
        );
        assert_abs_diff_eq!(
            Angle::from_deg(-90.).deg(),
            Angle::from_radian(-1.0 * PI / 2. + 2. * PI).deg()
        );
        assert_abs_diff_eq!(
            Angle::from_deg(-90.).radian(),
            Angle::from_radian(-1.0 * PI / 2.).radian()
        );

        assert_eq!(Angle::from_deg(90.0).deg_min_sec(), (90, 0, 0));
        assert_eq!(
            Angle::from_deg_min_sec(12, 13, 15).deg_min_sec(),
            (12, 13, 15)
        );
    }
}
