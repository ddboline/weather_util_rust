use anyhow::Error;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use stack_string::{format_sstr, StackString};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as FmtWrite,
};

use crate::{
    humidity::Humidity,
    precipitation::Precipitation,
    pressure::Pressure,
    temperature::Temperature,
    timestamp,
    timezone::TimeZone,
    weather_data::{Rain, Snow, WeatherCond},
};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
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
pub struct ForecastEntry {
    #[serde(with = "timestamp")]
    pub dt: DateTime<Utc>,
    pub main: ForecastMain,
    pub weather: Vec<WeatherCond>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<Rain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snow: Option<Snow>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
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
    /// # use std::collections::BTreeSet;
    /// # use stack_string::StackString;
    /// use weather_util_rust::weather_forecast::WeatherForecast;
    /// use weather_util_rust::temperature::Temperature;
    /// use weather_util_rust::precipitation::Precipitation;
    /// # fn main() -> Result<(), Error> {
    /// # let mut buf = String::new();
    /// # let mut f = File::open("tests/forecast.json")?;
    /// # f.read_to_string(&mut buf)?;
    /// let data: WeatherForecast = serde_json::from_str(&buf)?;
    ///
    /// let high_low = data.get_high_low();
    /// assert_eq!(high_low.len(), 6);
    /// let date: NaiveDate = "2022-02-27".parse()?;
    /// let icons: BTreeSet<StackString> = ["04n"].iter().map(|s| (*s).into()).collect();
    /// assert_eq!(
    ///     high_low.get(&date),
    ///     Some(
    ///         &(
    ///             Temperature::try_from(276.76)?,
    ///             Temperature::try_from(275.01)?,
    ///             Precipitation::default(),
    ///             Precipitation::default(),
    ///             icons,
    ///         )
    ///     )
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_high_low(
        &self,
    ) -> BTreeMap<
        NaiveDate,
        (
            Temperature,
            Temperature,
            Precipitation,
            Precipitation,
            BTreeSet<StackString>,
        ),
    > {
        let fo: FixedOffset = self.city.timezone.into();
        self.list.iter().fold(BTreeMap::new(), |mut hmap, entry| {
            let date = entry.dt.with_timezone(&fo).date().naive_local();
            let high = entry.main.temp_max;
            let low = entry.main.temp_min;
            let rain = if let Some(rain) = &entry.rain {
                rain.three_hour.unwrap_or_default()
            } else {
                Precipitation::default()
            };
            let snow = if let Some(snow) = &entry.snow {
                snow.three_hour.unwrap_or_default()
            } else {
                Precipitation::default()
            };
            let mut icons: BTreeSet<StackString> =
                entry.weather.iter().map(|w| w.icon.clone()).collect();

            if let Some((h, l, r, s, i)) = hmap.get(&date) {
                let high = if high > *h { high } else { *h };
                let low = if low < *l { low } else { *l };
                let rain = *r + rain;
                let snow = *s + snow;
                for ic in i {
                    if !icons.contains(ic) {
                        icons.insert(ic.clone());
                    }
                }

                if (high, low) != (*h, *l) {
                    hmap.insert(date, (high, low, rain, snow, icons));
                }
            } else {
                hmap.insert(date, (high, low, rain, snow, icons));
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
    /// let buf = data.get_forecast()?.join("");
    ///
    /// assert!(buf.starts_with("\nForecast:"), buf);
    /// assert!(buf.contains("2022-02-27 High: 38.5 F / 3.6 C"));
    /// assert!(buf.contains("Low: 35.3 F / 1.9 C"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_forecast(&self) -> Result<Vec<String>, Error> {
        let mut output = vec!["\nForecast:\n".into()];
        output.extend(self.get_high_low().into_iter().map(|(d, (h, l, r, s, _))| {
            let high = format_sstr!("High: {:0.1} F / {:0.1} C", h.fahrenheit(), h.celcius());
            let low = format_sstr!("Low: {:0.1} F / {:0.1} C", l.fahrenheit(), l.celcius());
            let mut rain_snow = StackString::new();
            if r.millimeters() > 0.0 {
                rain_snow.push_str(&format_sstr!("Rain {:0.2} in", r.inches()));
            }
            if s.millimeters() > 0.0 {
                if !rain_snow.is_empty() {
                    rain_snow.push_str("\t");
                }
                rain_snow.push_str(&format_sstr!("Snow {:0.2} in", s.inches()));
            }
            format!("\t{d} {high:25} {low:25} {rain_snow:25}\n")
        }));
        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use chrono::NaiveDate;
    use stack_string::StackString;
    use std::{collections::BTreeSet, convert::TryFrom};

    use crate::{
        precipitation::Precipitation, temperature::Temperature, weather_forecast::WeatherForecast,
    };

    #[test]
    fn test_get_high_low() -> Result<(), Error> {
        let buf = include_str!("../tests/forecast.json");
        let data: WeatherForecast = serde_json::from_str(&buf)?;
        let high_low = data.get_high_low();
        assert_eq!(high_low.len(), 6);
        let date: NaiveDate = "2022-02-27".parse()?;
        let icons: BTreeSet<StackString> = ["04n"].iter().map(|s| (*s).into()).collect();
        assert_eq!(
            high_low.get(&date),
            Some(&(
                Temperature::try_from(276.76)?,
                Temperature::try_from(275.01)?,
                Precipitation::default(),
                Precipitation::default(),
                icons,
            ))
        );

        Ok(())
    }

    #[test]
    fn test_get_forecast() -> Result<(), Error> {
        let buf = include_str!("../tests/forecast.json");
        let data: WeatherForecast = serde_json::from_str(&buf)?;
        let forecasts = data.get_forecast()?;
        let buf = forecasts.join("");
        println!("{}", buf);
        assert!(buf.starts_with("\nForecast:"));
        assert!(buf.contains("2022-02-27 High: 38.5 F / 3.6 C"));
        assert!(buf.contains("Low: 35.3 F / 1.9 C"));
        for f in forecasts {
            println!("{}", f.len());
        }
        Ok(())
    }
}
