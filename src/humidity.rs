use nutype::nutype;

/// Relative Humidity as Percent
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 100),
    derive(
        Display,
        TryFrom,
        AsRef,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        PartialEq,
        Debug,
        Into
    )
)]
pub struct Humidity(i64);

impl Default for Humidity {
    fn default() -> Self {
        Self::try_new(0).unwrap()
    }
}

#[cfg(test)]
mod test {

    use crate::{humidity::Humidity, Error};

    #[test]
    fn test_humidity() -> Result<(), Error> {
        let h = Humidity::try_new(86)?;
        let v: i64 = h.into_inner();
        assert_eq!(v, 86);

        let h = Humidity::try_new(-86).map_err(Into::<Error>::into);
        assert_eq!(
            &format!("{h:?}"),
            "Err(HumidityError(GreaterOrEqualViolated))"
        );
        Ok(())
    }
}
