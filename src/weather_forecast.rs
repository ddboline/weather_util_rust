use anyhow::Error;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::Write};

use crate::{
    humidity::Humidity, latitude::Latitude, longitude::Longitude, precipitation::Precipitation,
    pressure::Pressure, temperature::Temperature, timestamp, timezone::TimeZone,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForecastMain {
    pub temp: Temperature,
    pub feels_like: Temperature,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: Pressure,
    pub sea_level: Pressure,
    pub grnd_level: Pressure,
    pub humidity: Humidity,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rain {
    #[serde(alias = "3h")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Snow {
    #[serde(alias = "3h")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForecastEntry {
    #[serde(with = "timestamp")]
    pub dt: DateTime<Utc>,
    pub main: ForecastMain,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CityEntry {
    pub timezone: TimeZone,
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
    pub fn get_high_low(
        &self,
    ) -> BTreeMap<NaiveDate, (Temperature, Temperature, Precipitation, Precipitation)> {
        let fo: FixedOffset = self.city.timezone.into();
        self.list.iter().fold(BTreeMap::new(), |mut hmap, entry| {
            let date = entry.dt.with_timezone(&fo).date().naive_local();
            let high = entry.main.temp_max;
            let low = entry.main.temp_min;
            let rain = if let Some(rain) = &entry.rain {
                rain.three_hour.unwrap_or(Precipitation::default())
            } else {
                Precipitation::default()
            };
            let snow = if let Some(snow) = &entry.snow {
                snow.three_hour.unwrap_or(Precipitation::default())
            } else {
                Precipitation::default()
            };

            if let Some((h, l, r, s)) = hmap.get(&date) {
                let high = if high > *h { high } else { *h };
                let low = if low < *l { low } else { *l };
                let rain = *r + rain;
                let snow = *s + snow;

                if (high, low) != (*h, *l) {
                    hmap.insert(date, (high, low, rain, snow));
                }
            } else {
                hmap.insert(date, (high, low, rain, snow));
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
            .map(|(d, (h, l, r, s))| {
                writeln!(
                    buf,
                    "\t{} {:25} {:25} {:25}",
                    d,
                    format!("High: {:0.1} F / {:0.1} C", h.fahrenheit(), h.celcius(),),
                    format!("Low: {:0.1} F / {:0.1} C", l.fahrenheit(), l.celcius(),),
                    format!(
                        "{}{}",
                        if r.millimeters() > 0.0 {
                            format!("Rain {:0.2}", r.inches())
                        } else {
                            "".to_string()
                        },
                        if s.millimeters() > 0.0 {
                            format!("Snow {:0.2} in", s.inches())
                        } else {
                            "".to_string()
                        },
                    )
                )
                .map(|_| ())
                .map_err(Into::into)
            })
            .collect()
    }
}
