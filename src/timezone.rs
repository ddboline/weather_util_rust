use anyhow::{format_err, Error};
use derive_more::{Display, Into};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use time::UtcOffset;

/// Direction in degrees
#[derive(
    Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display, Default,
)]
#[serde(into = "i32", try_from = "i32")]
pub struct TimeZone(i32);

impl TryFrom<i32> for TimeZone {
    type Error = Error;
    fn try_from(item: i32) -> Result<Self, Self::Error> {
        if item > -86400 && item < 86400 {
            Ok(Self(item))
        } else {
            Err(format_err!("{item} is not a valid timezone"))
        }
    }
}

impl From<TimeZone> for UtcOffset {
    fn from(z: TimeZone) -> Self {
        match Self::from_whole_seconds(z.0) {
            Ok(offset) => offset,
            Err(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use std::convert::TryFrom;
    use time::UtcOffset;

    use crate::timezone::TimeZone;

    #[test]
    fn test_timezone() -> Result<(), Error> {
        let t = TimeZone::try_from(4 * 3600)?;
        let offset: i32 = t.into();
        assert_eq!(offset, 4 * 3600);
        let offset: UtcOffset = t.into();
        assert_eq!(offset, UtcOffset::from_whole_seconds(4 * 3600).unwrap());

        let t = TimeZone::try_from(100_000);
        assert!(t.is_err());
        assert_eq!(
            t.err().unwrap().to_string(),
            format!("{} is not a valid timezone", 100_000)
        );
        Ok(())
    }
}
