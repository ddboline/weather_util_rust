use anyhow::Error;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::Write};

use crate::{latitude::Latitude, longitude::Longitude, temperature::Temperature, timestamp};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForecastMain {
    pub temp: Temperature,
    pub feels_like: f64,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
    pub humidity: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForecastEntry {
    #[serde(with = "timestamp")]
    pub dt: DateTime<Utc>,
    pub main: ForecastMain,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CityEntry {
    pub timezone: i32,
    #[serde(with = "timestamp")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "timestamp")]
    pub sunset: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WeatherForecast {
    pub list: Vec<ForecastEntry>,
    pub city: CityEntry,
}

impl WeatherForecast {
    /// Get Map of Date to High/Low temperatures
    /// ```
    /// # use anyhow::Error;
    /// # use std::io::{stdout, Write, Read};
    /// # use std::fs::File;
    /// # use std::convert::TryFrom;
    /// # use chrono::NaiveDate;
    /// use weather_util_rust::weather_forecast::WeatherForecast;
    /// use weather_util_rust::temperature::Temperature;
    /// # fn main() -> Result<(), Error> {
    /// # let mut buf = String::new();
    /// # let mut f = File::open("tests/forecast.json")?;
    /// # f.read_to_string(&mut buf)?;
    /// let data: WeatherForecast = serde_json::from_str(&buf)?;
    ///
    /// let high_low = data.get_high_low();
    /// assert_eq!(high_low.len(), 6);
    /// let date: NaiveDate = "2020-01-21".parse()?;
    /// assert_eq!(
    ///     high_low.get(&date),
    ///     Some(
    ///         &(
    ///             Temperature::try_from(272.65)?,
    ///             Temperature::try_from(266.76)?
    ///         )
    ///     )
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_high_low(&self) -> BTreeMap<NaiveDate, (Temperature, Temperature)> {
        let fo = FixedOffset::east(self.city.timezone);
        self.list.iter().fold(BTreeMap::new(), |mut hmap, entry| {
            let date = entry.dt.with_timezone(&fo).date().naive_local();
            let high = entry.main.temp_max;
            let low = entry.main.temp_min;

            if let Some((h, l)) = hmap.get(&date) {
                let high = if high > *h { high } else { *h };
                let low = if low < *l { low } else { *l };

                if (high, low) != (*h, *l) {
                    hmap.insert(date, (high, low));
                }
            } else {
                hmap.insert(date, (high, low));
            }
            hmap
        })
    }

    /// Get High and Low Temperatures for the Next Few Days
    /// ```
    /// # use anyhow::Error;
    /// # use std::io::{stdout, Write, Read};
    /// # use std::fs::File;
    /// # use std::convert::TryFrom;
    /// # use chrono::NaiveDate;
    /// use weather_util_rust::weather_forecast::WeatherForecast;
    /// # fn main() -> Result<(), Error> {
    /// # let mut buf = String::new();
    /// # let mut f = File::open("tests/forecast.json")?;
    /// # f.read_to_string(&mut buf)?;
    /// let data: WeatherForecast = serde_json::from_str(&buf)?;
    ///
    /// let mut buf = Vec::new();
    /// data.get_forecast(&mut buf)?;
    ///
    /// let buf = String::from_utf8(buf)?;
    /// assert!(buf.starts_with("\nForecast:"), buf);
    /// assert!(buf.contains("2020-01-23 High: 37.72 F / 3.18 C"));
    /// assert!(buf.contains("Low: 30.07 F / -1.07 C"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_forecast<T: Write>(&self, buf: &mut T) -> Result<(), Error> {
        writeln!(buf, "\nForecast:")?;
        self.get_high_low()
            .into_iter()
            .map(|(d, (h, l))| {
                writeln!(
                    buf,
                    "\t{} {:30} {:30}",
                    d,
                    format!("High: {:0.2} F / {:0.2} C", h.fahrenheit(), h.celcius(),),
                    format!("Low: {:0.2} F / {:0.2} C", l.fahrenheit(), l.celcius(),),
                )
                .map(|_| ())
                .map_err(Into::into)
            })
            .collect()
    }
}
