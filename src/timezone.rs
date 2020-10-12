use anyhow::{format_err, Error};
use chrono::offset::FixedOffset;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Direction in degrees
#[derive(Into, Debug, PartialEq, Copy, Clone, PartialOrd, Serialize, Deserialize, Display)]
#[serde(into = "i32", try_from = "i32")]
pub struct TimeZone(i32);

impl TryFrom<i32> for TimeZone {
    type Error = Error;
    fn try_from(item: i32) -> Result<Self, Self::Error> {
        if item > -86400 && item < 86400 {
            Ok(Self(item))
        } else {
            Err(format_err!("{} is not a valid timezone", item))
        }
    }
}

impl std::convert::Into<FixedOffset> for TimeZone {
    fn into(self) -> FixedOffset {
        FixedOffset::east(self.0)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use chrono::offset::FixedOffset;
    use std::convert::TryFrom;

    use crate::timezone::TimeZone;

    #[test]
    fn test_timezone() -> Result<(), Error> {
        let t = TimeZone::try_from(4 * 3600)?;
        let offset: i32 = t.into();
        assert_eq!(offset, 4 * 3600);
        let offset: FixedOffset = t.into();
        assert_eq!(offset, FixedOffset::east(4 * 3600));

        let t = TimeZone::try_from(100_000);
        assert!(t.is_err());
        assert_eq!(
            t.err().unwrap().to_string(),
            format!("{} is not a valid timezone", 100_000)
        );
        Ok(())
    }
}
