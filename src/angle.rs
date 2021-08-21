use derive_more::{Into, Display};
use rust_decimal::prelude::ToPrimitive;
use serde::{Serialize, Deserialize};
use std::{f64::consts::PI};
use std::fmt;
use std::str::FromStr;
use anyhow::Error;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use std::convert::TryFrom;

const DEGREE_PER_TURN: i64 = 360;
const MINUTES_PER_DEGREE: i64 = 60;
const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_DEGREE: i64 = SECONDS_PER_MINUTE * MINUTES_PER_DEGREE;
const SECONDS_PER_TURN: i64 = DEGREE_PER_TURN * MINUTES_PER_DEGREE * SECONDS_PER_MINUTE;
const RADIANS_PER_TURN: f64 = 2.0 * PI;

/// Angle in arcseconds
#[derive(Into, Debug, Copy, Clone, PartialOrd, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(into="Decimal", from="Decimal")]
pub struct Angle(i64);

impl From<i64> for Angle {
    fn from(item: i64) -> Self {
        if item >= 0 {
            Self(item % SECONDS_PER_TURN)
        } else {
            Self((item % SECONDS_PER_TURN) + SECONDS_PER_TURN)
        }
    }
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

impl From<Decimal> for Angle {
    fn from(item: Decimal) -> Self {
        let sec = item * Decimal::from_i64(SECONDS_PER_DEGREE).expect("Unexpected");
        Self(sec.round().to_i64().expect("unexpected"))
    }
}

impl From<Angle> for Decimal {
    fn from(item: Angle) -> Self {
        let sec = Decimal::from_i64(item.0).expect("Unexpected");
        sec / Decimal::from_i64(SECONDS_PER_DEGREE).expect("Unexpected")
    }
}

// impl From<f64> for Angle {
//     fn from(item: f64) -> Self {
//         Self::from_deg(item)
//     }
// }

// impl From<Angle> for f64 {
//     fn from(item: Angle) -> Self {
//         item.deg()
//     }
// }

impl Angle {
    pub fn from_deg_min_sec(deg: i16, min: u8, sec: u8) -> Self {
        let deg = deg as i64 % 360;
        let min = min as i64 % 60;
        let sec = sec as i64 % 60;
        Self::from((deg % 360) * SECONDS_PER_DEGREE + min * SECONDS_PER_MINUTE + sec)
    }

    pub fn from_deg(deg: f64) -> Self {
        Self::from((deg * SECONDS_PER_DEGREE as f64) as i64)
    }

    pub fn from_radian(rad: f64) -> Self {
        Self::from(((rad / RADIANS_PER_TURN) * SECONDS_PER_TURN as f64) as i64)
    }

    #[inline]
    pub fn deg(self) -> f64 {
        self.0 as f64 / SECONDS_PER_DEGREE as f64
    }

    pub fn deg_min_sec(self) -> (i16, u8, u8) {
        let deg = (self.0 / SECONDS_PER_DEGREE) % 360;
        let min = self.0 / SECONDS_PER_MINUTE - deg * MINUTES_PER_DEGREE;
        let sec = self.0 - min * SECONDS_PER_MINUTE - deg * SECONDS_PER_DEGREE;
        let min = if min > 0 {
            min % 60
        } else {
            (min % 60) + 60
        };
        let sec = if sec > 0 {
            sec % 60
        } else {
            (sec % 60) + 60
        };
        (deg as i16, min as u8, sec as u8)
    }

    #[inline]
    pub fn radian(self) -> f64 {
        self.0 as f64 * RADIANS_PER_TURN / SECONDS_PER_TURN as f64
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
        assert_eq!(Angle::from_deg_min_sec(12, 13, 15).deg_min_sec(), (12, 13, 15));
    }
}
