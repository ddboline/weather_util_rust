use derive_more::Into;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::Error;

/// Angle in degrees
#[derive(Into, Debug, Copy, Clone, PartialOrd, Serialize, Deserialize, Default)]
#[serde(into = "f64", from = "f64")]
pub struct Angle {
    degree: i16,
    minute: u8,
    second: u8,
    subsec: f32,
}

impl Angle {
    #[inline]
    #[must_use]
    fn sign(self) -> i8 {
        if self.degree >= 0 { 1 } else { -1 }
    }

    #[must_use]
    pub fn from_deg_min_sec_subsec(degree: i16, minute: u8, second: u8, subsec: f32) -> Self {
        let sign = if degree >= 0 { 1 } else { -1 };
        let abs_degree = degree.abs() % 360;
        let degree = sign * abs_degree;
        let minute = minute % 60;
        let second = second % 60;
        let subsec = subsec.abs() % 1.0;
        Self {
            degree,
            minute,
            second,
            subsec,
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn from_deg(deg: f64) -> Self {
        let sign = if deg >= 0.0 { 1 } else { -1 };
        let abs_deg = deg.abs() % 360.0;
        let abs_degree = abs_deg.floor();
        let minute = (((abs_deg * 60.0) - (abs_degree * 60.0)) % 60.0).floor();
        let sec = ((abs_deg * 3600.0) - minute * 60.0 - abs_degree * 3600.0) % 60.0;
        let degree = sign * abs_deg as i16;
        let minute = minute as u8;
        let second = sec as u8;
        let subsec = sec as f32 - second as f32;
        Self {
            degree,
            minute,
            second,
            subsec,
        }
    }

    #[must_use]
    pub fn from_radian(rad: f64) -> Self {
        Self::from(rad.to_degrees())
    }

    #[inline]
    #[must_use]
    pub fn deg(self) -> f64 {
        self.sign() as f64
            * (self.degree.abs() as f64
                + (self.minute as f64 / 60.0)
                + (self.second as f64 / 3600.0)
                + (self.subsec as f64 / 3600.0))
    }

    #[inline]
    #[must_use]
    pub fn deg_min_sec_subsec(self) -> (i16, u8, u8, f32) {
        (self.degree, self.minute, self.second, self.subsec)
    }

    #[inline]
    #[must_use]
    pub fn radian(self) -> f64 {
        self.deg().to_radians()
    }
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

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use std::{f64::consts::PI, mem::size_of};

    use crate::{Error, angle::Angle};

    #[test]
    fn test_sizeof() {
        assert_eq!(size_of::<f64>(), 8);
        assert_eq!(size_of::<Angle>(), 8);
    }

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
        assert_eq!(Angle::from_deg(90.0).deg_min_sec_subsec(), (90, 0, 0, 0.0));
        let x: f64 = Angle::from_deg(90.0).into();
        assert_eq!(90.0, x);
        assert_eq!(
            Angle::from_deg_min_sec_subsec(12, 13, 15, 0.0).deg_min_sec_subsec(),
            (12, 13, 15, 0.0)
        );
        assert_eq!(
            Angle::from_deg(-42.3).deg_min_sec_subsec(),
            (-42, 18, 0, 0.0)
        );
        assert_eq!(-42.3, Angle::from_deg(-42.3).deg());
        assert_eq!(
            Angle::from_deg(-42.12344562124).deg_min_sec_subsec(),
            (-42, 7, 24, 0.40423584)
        );
        assert!(
            (Angle::from_deg_min_sec_subsec(-12, 5, 13, 0.5).deg() + 12.087083).abs() < 0.000001
        );
    }

    #[test]
    fn test_parse() -> Result<(), Error> {
        let a = Angle::from_deg_min_sec_subsec(42, 0, 0, 0.0);
        let b: Angle = "42.0".parse()?;
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn test_real_lat() -> Result<(), Error> {
        let a = Angle::from_deg(40.7633578);
        assert_eq!(a.degree, 40);
        assert_eq!(a.minute, 45);
        assert_eq!(a.second, 48);
        assert!((a.subsec - 0.08808136).abs() < 0.00000001);
        Ok(())
    }
}
