use anyhow::{format_err, Error};
use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Relative Humidity as Percent
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display, Default)]
#[serde(into = "i64", try_from = "i64")]
pub struct Humidity(i64);

impl TryFrom<i64> for Humidity {
    type Error = Error;
    fn try_from(item: i64) -> Result<Self, Self::Error> {
        if (0..=100).contains(&item) {
            Ok(Self(item))
        } else {
            Err(format_err!("{item} is not a valid relative humidity"))
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use std::convert::TryFrom;

    use crate::humidity::Humidity;

    #[test]
    fn test_humidity() -> Result<(), Error> {
        let h = Humidity::try_from(86)?;
        let v: i64 = h.into();
        assert_eq!(v, 86);

        let h = Humidity::try_from(-86);
        assert!(h.is_err());
        assert_eq!(
            h.err().unwrap().to_string(),
            "-86 is not a valid relative humidity"
        );
        Ok(())
    }
}
