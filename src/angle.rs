use anyhow::Error;
use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    f64::consts::PI,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

const RADIANS_PER_TURN: f64 = 2.0 * PI;

/// Angle in degrees
#[derive(Into, Debug, Copy, Clone, PartialOrd, Serialize, Deserialize)]
#[serde(into = "f64", from = "f64")]
pub struct Angle {
    degree: i16,
    minute: i8,
    second: i8,
    subsec: f32,
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        ((self.degree - other.degree) % 360 == 0)
            && (self.minute == other.minute)
            && (self.second == other.second)
    }
}

impl Eq for Angle {}

impl Hash for Angle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.degree.hash(state);
        self.minute.hash(state);
        self.second.hash(state);
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
    pub fn from_deg_min_sec(degree: i16, minute: i8, sec: f32) -> Self {
        let degree = degree % 360;
        let minute = minute % 60;
        let second = (sec % 60.0) as i8;
        let subsec = (sec % 60.0) - sec as f32;
        Self {
            degree,
            minute,
            second,
            subsec,
        }
    }

    pub fn from_deg(deg: f64) -> Self {
        let deg = deg % 360.0;
        let degree = deg as i64;
        let minute = (deg * 60.0) as i64 - (degree * 60);
        let sec = (deg * 3600.0) - minute as f64 * 60.0 - degree as f64 * 3600.0;
        let degree = deg as i16;
        let minute = (minute % 60) as i8;
        let sec = sec % 60.0;
        let second = sec as i8;
        let subsec = sec as f32 - second as f32;
        Self {
            degree,
            minute,
            second,
            subsec,
        }
    }

    pub fn from_radian(rad: f64) -> Self {
        Self::from(rad * 360.0 / RADIANS_PER_TURN)
    }

    #[inline]
    pub fn deg(self) -> f64 {
        self.degree as f64 + (self.minute as f64 / 60.0) + (self.second as f64 / 3600.0)
    }

    pub fn deg_min_sec(self) -> (i16, i8, f64) {
        (
            self.degree,
            self.minute,
            self.second as f64 + self.subsec as f64,
        )
    }

    #[inline]
    pub fn radian(self) -> f64 {
        self.deg() * RADIANS_PER_TURN / 360.0
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    use crate::angle::Angle;

    #[test]
    fn test_direction() {
        assert_eq!(Angle::from_deg(90.), Angle::from_deg(90. + 360.));
        assert_eq!(Angle::from_deg(90.), Angle::from_radian(PI / 2.));
        assert_eq!(Angle::from_deg(90.), Angle::from_radian(PI / 2. + 2. * PI));
        assert_abs_diff_eq!(
            Angle::from_deg(90.).radian(),
            Angle::from_radian(PI / 2.).radian()
        );

        assert_eq!(Angle::from_deg(-90.), Angle::from_deg(-90. + 360.));
        assert_abs_diff_eq!(
            Angle::from_deg(-90.).deg(),
            Angle::from_radian(-1.0 * PI / 2.).deg()
        );
        assert_eq!(
            Angle::from_deg(-90.),
            Angle::from_radian(-1.0 * PI / 2. + 2. * PI)
        );
        assert_abs_diff_eq!(
            Angle::from_deg(-90.).radian(),
            Angle::from_radian(-1.0 * PI / 2.).radian()
        );
        assert_eq!(Angle::from_deg(90.0).deg_min_sec(), (90, 0, 0.0));
        let x: f64 = Angle::from_deg(90.0).into();
        assert_eq!(90.0, x);
        assert_eq!(
            Angle::from_deg_min_sec(12, 13, 15.0).deg_min_sec(),
            (12, 13, 15.0)
        );
        assert_eq!(Angle::from_deg(-42.3).deg_min_sec(), (-42, -18, 0.0));
        assert_eq!(-42.3, Angle::from_deg(-42.3).deg());
    }

    #[test]
    fn test_parse() -> Result<(), Error> {
        let a = Angle::from_deg_min_sec(42, 0, 0.0);
        let b: Angle = "42.0".parse()?;
        assert_eq!(a, b);
        Ok(())
    }
}
