use anyhow::{format_err, Error};
use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, f64::consts::PI};

/// Direction in degrees
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display)]
#[serde(into = "f64", from = "f64")]
pub struct Direction(f64);

impl From<f64> for Direction {
    fn from(item: f64) -> Self {
        if item >= 0.0 {
            Self(item % 360.0)
        } else {
            Self((item % 360.0) + 360.0)
        }
    }
}

impl Direction {
    pub fn from_deg(deg: f64) -> Self {
        Self::from(deg)
    }

    pub fn from_radian(rad: f64) -> Self {
        Self::from(rad * 180.0 / PI)
    }

    pub fn deg(self) -> f64 {
        self.0
    }

    pub fn radian(self) -> f64 {
        self.0 * PI / 180.0
    }
}
