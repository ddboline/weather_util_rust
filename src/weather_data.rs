use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    direction::Direction, distance::Distance, humidity::Humidity, latitude::Latitude,
    longitude::Longitude, precipitation::Precipitation, pressure::Pressure, speed::Speed,
    temperature::Temperature, timestamp, timezone::TimeZone,
    StringType, format_string,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Coord {
    pub lon: Longitude,
    pub lat: Latitude,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WeatherCond {
    pub id: usize,
    pub main: StringType,
    pub description: StringType,
    pub icon: StringType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct WeatherMain {
    pub temp: Temperature,
    pub feels_like: Temperature,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: Pressure,
    pub humidity: Humidity,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct Wind {
    pub speed: Speed,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deg: Option<Direction>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<StringType>,
    #[serde(with = "timestamp")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "timestamp")]
    pub sunset: DateTime<Utc>,
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            country: None,
            sunrise: Utc::now(),
            sunset: Utc::now(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Rain {
    #[serde(alias = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Snow {
    #[serde(alias = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<WeatherCond>,
    pub base: StringType,
    pub main: WeatherMain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Distance>,
    pub wind: Wind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<Rain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snow: Option<Snow>,
    #[serde(with = "timestamp")]
    pub dt: DateTime<Utc>,
    pub sys: Sys,
    pub timezone: TimeZone,
    pub name: StringType,
}

impl Default for WeatherData {
    fn default() -> Self {
        Self {
            coord: Coord::default(),
            weather: Vec::new(),
            base: "".into(),
            main: WeatherMain::default(),
            visibility: None,
            wind: Wind::default(),
            rain: None,
            snow: None,
            dt: Utc::now(),
            sys: Sys::default(),
            timezone: TimeZone::default(),
            name: "".into(),
        }
    }
}

impl WeatherData {
    #[must_use]
    pub fn get_offset(&self) -> FixedOffset {
        self.timezone.into()
    }

    #[must_use]
    pub fn get_dt(&self) -> DateTime<FixedOffset> {
        self.dt.with_timezone(&self.get_offset())
    }

    #[must_use]
    pub fn get_sunrise(&self) -> DateTime<FixedOffset> {
        self.sys.sunrise.with_timezone(&self.get_offset())
    }

    #[must_use]
    pub fn get_sunset(&self) -> DateTime<FixedOffset> {
        self.sys.sunset.with_timezone(&self.get_offset())
    }

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
    /// let buf = data.get_current_conditions();
    ///
    /// assert!(buf.starts_with("Current conditions Astoria US 40.76"));
    /// assert!(buf.contains("Temperature: 38.50 F (3.61 C)"));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_current_conditions(&self) -> StringType {
        let fo: FixedOffset = self.timezone.into();
        let dt = self.dt.with_timezone(&fo);
        let sunrise = self.sys.sunrise.with_timezone(&fo);
        let sunset = self.sys.sunset.with_timezone(&fo);
        let country_str = if let Some(country) = &self.sys.country {
            let name = &self.name;
            format_string!("{name} {country}")
        } else {
            StringType::new()
        };
        let lat_lon = format_string!("{:0.5}N {:0.5}E", self.coord.lat, self.coord.lon);
        let dt_str = format_string!("Last Updated {dt}");
        let temp_str = format_string!(
            "\tTemperature: {f:0.2} F ({c:0.2} C)",
            f = self.main.temp.fahrenheit(),
            c = self.main.temp.celcius(),
        );
        let humidity_str = format_string!("\tRelative Humidity: {}%", self.main.humidity);
        let wind_str = format_string!(
            "\tWind: {d} degrees at {s:0.2} mph",
            d = self.wind.deg.unwrap_or_else(|| 0.0.into()),
            s = self.wind.speed.mph(),
        );
        let conditions_str = format_string!("\tConditions: {}", self.weather[0].description);
        let sunrise_str = format_string!("\tSunrise: {sunrise}");
        let sunset_str = format_string!("\tSunset: {sunset}");
        let rain_str = if let Some(rain) = &self.rain {
            format_string!(
                "\n\tRain: {} in",
                rain.three_hour.map_or(0.0, Precipitation::inches)
            )
        } else {
            StringType::new()
        };
        let snow_str = if let Some(snow) = &self.snow {
            format_string!(
                "\n\tSnow: {} in",
                snow.three_hour.map_or(0.0, Precipitation::inches)
            )
        } else {
            StringType::new()
        };
        format_string!(
            "Current conditions {} {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}{}\n",
            country_str,
            lat_lon,
            dt_str,
            temp_str,
            humidity_str,
            wind_str,
            conditions_str,
            sunrise_str,
            sunset_str,
            rain_str,
            snow_str,
        )
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

        let buf = data.get_current_conditions();

        assert!(buf.starts_with("Current conditions Astoria US 40.76"));
        assert!(buf.contains("Temperature: 38.50 F (3.61 C)"));
        println!("{} {} {}", buf.len(), data.name, data.name.len());
        Ok(())
    }
}
