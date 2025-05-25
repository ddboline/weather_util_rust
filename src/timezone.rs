use nutype::nutype;
use time::UtcOffset;

/// Direction in degrees
#[nutype(validate(greater_or_equal=-86400, less_or_equal=86400), derive(Display, TryFrom, AsRef, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug, Into,))]
pub struct TimeZone(i32);

impl Default for TimeZone {
    fn default() -> Self {
        Self::try_new(0).unwrap()
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

    use crate::{Error, timezone::TimeZone};

    #[test]
    fn test_timezone() -> Result<(), Error> {
        let t = TimeZone::try_new(4 * 3600)?;
        let offset: i32 = t.into_inner();
        assert_eq!(offset, 4 * 3600);
        let offset: UtcOffset = t.into();
        assert_eq!(offset, UtcOffset::from_whole_seconds(4 * 3600).unwrap());

        let t = TimeZone::try_from(100_000).map_err(Into::<Error>::into);
        assert_eq!(&format!("{t:?}"), "Err(TimeZoneError(LessOrEqualViolated))");
        Ok(())
    }
}
