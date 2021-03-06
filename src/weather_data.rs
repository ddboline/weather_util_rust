use anyhow::Error;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use rweb::Schema;
use serde::{Deserialize, Serialize};
use stack_string::StackString;
use std::{collections::BTreeMap, io::Write};

use crate::{
    datetime_wrapper::DateTimeWrapper, direction::Direction, distance::Distance,
    humidity::Humidity, latitude::Latitude, longitude::Longitude, precipitation::Precipitation,
    pressure::Pressure, speed::Speed, temperature::Temperature, timestamp, timezone::TimeZone,
};

#[derive(Serialize, Deserialize, Debug, Clone, Schema)]
pub struct Coord {
    pub lon: Longitude,
    pub lat: Latitude,
}

#[derive(Serialize, Deserialize, Debug, Clone, Schema)]
pub struct WeatherCond {
    pub main: StackString,
    pub description: StackString,
}

#[derive(Serialize, Deserialize, Debug, Clone, Schema)]
pub struct WeatherMain {
    pub temp: Temperature,
    pub feels_like: Temperature,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: Pressure,
    pub humidity: Humidity,
}

#[derive(Deserialize, Serialize, Debug, Clone, Schema)]
pub struct Wind {
    pub speed: Speed,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deg: Option<Direction>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Schema)]
pub struct Sys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<StackString>,
    #[serde(with = "timestamp")]
    pub sunrise: DateTimeWrapper,
    #[serde(with = "timestamp")]
    pub sunset: DateTimeWrapper,
}

#[derive(Deserialize, Serialize, Debug, Clone, Schema)]
pub struct Rain {
    #[serde(alias = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Schema)]
pub struct Snow {
    #[serde(alias = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Schema)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<WeatherCond>,
    pub base: StackString,
    pub main: WeatherMain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Distance>,
    pub wind: Wind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<Rain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snow: Option<Snow>,
    #[serde(with = "timestamp")]
    pub dt: DateTimeWrapper,
    pub sys: Sys,
    pub timezone: TimeZone,
    pub name: StackString,
}

impl WeatherData {
    /// Write out formatted information about current conditions for a mutable
    /// buffer.
    /// ```
    /// use weather_util_rust::weather_data::WeatherData;
    /// # use anyhow::Error;
    /// # use std::io::{stdout, Write, Read};
    /// # use std::fs::File;
    /// # fn main() -> Result<(), Error> {
    /// # let mut buf = String::new();
    /// # let mut f = File::open("tests/weather.json")?;
    /// # f.read_to_string(&mut buf)?;
    /// let data: WeatherData = serde_json::from_str(&buf)?;
    ///
    /// let buf = data.get_current_conditions()?;
    ///
    /// assert!(buf.starts_with("Current conditions Astoria US 40.76"));
    /// assert!(buf.contains("Temperature: 41.05 F (5.03 C)"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_current_conditions(&self) -> Result<String, Error> {
        let fo: FixedOffset = self.timezone.into();
        let dt = self.dt.with_timezone(&fo);
        let sunrise = self.sys.sunrise.with_timezone(&fo);
        let sunset = self.sys.sunset.with_timezone(&fo);
        Ok(format!(
            "Current conditions {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}{}\n",
            if let Some(country) = &self.sys.country {
                format!("{} {}", self.name, country)
            } else {
                "".to_string()
            },
            format!("{}N {}E", self.coord.lat, self.coord.lon),
            format!("Last Updated {}", dt,),
            format!(
                "\tTemperature: {:0.2} F ({:0.2} C)",
                self.main.temp.fahrenheit(),
                self.main.temp.celcius(),
            ),
            format!("\tRelative Humidity: {}%", self.main.humidity),
            format!(
                "\tWind: {} degrees at {:0.2} mph",
                self.wind.deg.unwrap_or_else(|| 0.0.into()),
                (self.wind.speed.mph())
            ),
            format!("\tConditions: {}", self.weather[0].description),
            format!("\tSunrise: {}", sunrise),
            format!("\tSunset: {}", sunset),
            if let Some(rain) = &self.rain {
                format!(
                    "\n\tRain: {} in",
                    rain.three_hour.map_or(0.0, Precipitation::inches)
                )
            } else {
                "".to_string()
            },
            if let Some(snow) = &self.snow {
                format!(
                    "\n\tSnow: {} in",
                    snow.three_hour.map_or(0.0, Precipitation::inches)
                )
            } else {
                "".to_string()
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use anyhow::Error;

    use crate::weather_data::WeatherData;

    #[test]
    fn test_weather_data() -> Result<(), Error> {
        let buf = include_str!("../tests/weather.json");
        let data: WeatherData = serde_json::from_str(buf)?;

        let buf = data.get_current_conditions()?;

        assert!(buf.starts_with("Current conditions Astoria US 40.76"));
        assert!(buf.contains("Temperature: 41.05 F (5.03 C)"));

        Ok(())
    }
}
