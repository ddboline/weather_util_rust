use nutype::nutype;
use time::UtcOffset;

/// Direction in degrees
#[nutype(validate(min=-86400, max=86400))]
#[derive(*, Serialize, Deserialize, Display)]
pub struct TimeZone(i32);

impl Default for TimeZone {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

impl From<TimeZone> for UtcOffset {
    fn from(z: TimeZone) -> Self {
        match Self::from_whole_seconds(z.into_inner()) {
            Ok(offset) => offset,
            Err(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use time::UtcOffset;

    use crate::{format_string, timezone::TimeZone, Error};

    #[test]
    fn test_timezone() -> Result<(), Error> {
        let t = TimeZone::new(4 * 3600).map_err(|e| Error::InvalidValue(format_string!("{e}")))?;
        let offset: i32 = t.into_inner();
        assert_eq!(offset, 4 * 3600);
        let offset: UtcOffset = t.into();
        assert_eq!(offset, UtcOffset::from_whole_seconds(4 * 3600).unwrap());

        let t = TimeZone::try_from(100_000);
        assert!(t.is_err());
        Ok(())
    }
}
