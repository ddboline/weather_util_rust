use nutype::nutype;

/// Relative Humidity as Percent
#[nutype(validate(min=0, max=100))]
#[derive(*, Serialize, Deserialize, Display)]
pub struct Humidity(i64);

impl Default for Humidity {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

#[cfg(test)]
mod test {

    use crate::{format_string, humidity::Humidity, Error};

    #[test]
    fn test_humidity() -> Result<(), Error> {
        let h = Humidity::new(86).map_err(|e| Error::InvalidValue(format_string!("{e}")))?;
        let v: i64 = h.into_inner();
        assert_eq!(v, 86);

        let h = Humidity::new(-86);
        assert!(h.is_err());
        Ok(())
    }
}
