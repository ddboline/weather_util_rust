use serde::{Deserialize, Serialize};
use std::fmt::Write;
use time::{OffsetDateTime, UtcOffset};

use crate::{
    default_datetime, direction::Direction, distance::Distance, humidity::Humidity,
    latitude::Latitude, longitude::Longitude, precipitation::Precipitation, pressure::Pressure,
    speed::Speed, temperature::Temperature, timestamp, timezone::TimeZone, StringType,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Coord {
    pub lon: Longitude,
    pub lat: Latitude,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct WeatherCond {
    pub id: usize,
    pub main: StringType,
    pub description: StringType,
    pub icon: StringType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq)]
pub struct WeatherMain {
    pub temp: Temperature,
    pub feels_like: Temperature,
    pub temp_min: Temperature,
    pub temp_max: Temperature,
    pub pressure: Pressure,
    pub humidity: Humidity,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq)]
pub struct Wind {
    pub speed: Speed,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deg: Option<Direction>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Sys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<StringType>,
    #[serde(with = "timestamp")]
    pub sunrise: OffsetDateTime,
    #[serde(with = "timestamp")]
    pub sunset: OffsetDateTime,
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            country: None,
            sunrise: default_datetime(),
            sunset: default_datetime(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Rain {
    #[serde(alias = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Snow {
    #[serde(alias = "3h", skip_serializing_if = "Option::is_none")]
    pub three_hour: Option<Precipitation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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
    pub dt: OffsetDateTime,
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
            dt: default_datetime(),
            sys: Sys::default(),
            timezone: TimeZone::default(),
            name: "".into(),
        }
    }
}

impl WeatherData {
    #[must_use]
    pub fn get_offset(&self) -> UtcOffset {
        self.timezone.into()
    }

    #[must_use]
    pub fn get_dt(&self) -> OffsetDateTime {
        self.dt.to_offset(self.get_offset())
    }

    #[must_use]
    pub fn get_sunrise(&self) -> OffsetDateTime {
        self.sys.sunrise.to_offset(self.get_offset())
    }

    #[must_use]
    pub fn get_sunset(&self) -> OffsetDateTime {
        self.sys.sunset.to_offset(self.get_offset())
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
        let mut output: StringType = "Current conditions ".into();
        let fo: UtcOffset = self.timezone.into();
        let dt = self.dt.to_offset(fo);
        let sunrise = self.sys.sunrise.to_offset(fo);
        let sunset = self.sys.sunset.to_offset(fo);
        if let Some(country) = &self.sys.country {
            let name = &self.name;
            write!(output, "{name} {country} ").unwrap_or(());
        };
        writeln!(output, "{:0.5}N {:0.5}E", self.coord.lat, self.coord.lon).unwrap_or(());
        writeln!(output, "Last Updated {dt}").unwrap_or(());
        writeln!(
            output,
            "\tTemperature: {f:0.2} F ({c:0.2} C)",
            f = self.main.temp.fahrenheit(),
            c = self.main.temp.celcius(),
        )
        .unwrap_or(());
        writeln!(output, "\tRelative Humidity: {}%", self.main.humidity).unwrap_or(());
        writeln!(
            output,
            "\tWind: {d} degrees at {s:0.2} mph",
            d = self.wind.deg.unwrap_or_else(|| 0.0.into()),
            s = self.wind.speed.mph(),
        )
        .unwrap_or(());
        writeln!(
            output,
            "\tConditions: {}",
            self.weather.get(0).map_or_else(|| "", |w| &w.description)
        )
        .unwrap_or(());
        writeln!(output, "\tSunrise: {sunrise}").unwrap_or(());
        write!(output, "\tSunset: {sunset}").unwrap_or(());
        if let Some(rain) = &self.rain {
            write!(
                output,
                "\n\tRain: {} in",
                rain.three_hour.map_or(0.0, Precipitation::inches)
            )
            .unwrap_or(());
        };
        if let Some(snow) = &self.snow {
            write!(
                output,
                "\n\tSnow: {} in",
                snow.three_hour.map_or(0.0, Precipitation::inches)
            )
            .unwrap_or(());
        };
        output.push('\n');
        output
    }
}

#[cfg(test)]
mod test {
    use crate::{
        default_datetime,
        timezone::TimeZone,
        weather_data::{Coord, Sys, WeatherData, WeatherMain, Wind},
        Error,
    };
    use log::info;

    #[test]
    fn test_weather_data() -> Result<(), Error> {
        let buf = include_str!("../tests/weather.json");
        let data: WeatherData = serde_json::from_str(buf)?;

        let buf = data.get_current_conditions();

        assert!(buf.starts_with("Current conditions Astoria US 40.76"));
        assert!(buf.contains("Temperature: 38.50 F (3.61 C)"));
        info!("{} {} {}", buf.len(), data.name, data.name.len());
        Ok(())
    }

    #[test]
    fn test_default_sys() -> Result<(), Error> {
        let default_sys = Sys::default();
        assert_eq!(default_sys.country, None);
        assert_eq!(default_sys.sunrise, default_datetime());
        assert_eq!(default_sys.sunset, default_datetime());
        Ok(())
    }

    #[test]
    fn test_default_weather_data() -> Result<(), Error> {
        let default_data = WeatherData::default();
        assert_eq!(default_data.coord, Coord::default());
        assert_eq!(default_data.weather, Vec::new());
        assert_eq!(&default_data.base, "");
        assert_eq!(default_data.main, WeatherMain::default());
        assert_eq!(default_data.visibility, None);
        assert_eq!(default_data.wind, Wind::default());
        assert_eq!(default_data.rain, None);
        assert_eq!(default_data.snow, None);
        assert_eq!(default_data.dt, default_datetime());
        assert_eq!(default_data.sys, Sys::default());
        assert_eq!(default_data.timezone, TimeZone::default());
        assert_eq!(&default_data.name, "");
        let default_offset = default_data.get_offset();
        assert_eq!(default_data.get_offset(), TimeZone::default().into());
        assert_eq!(
            default_data.get_dt(),
            default_datetime().to_offset(default_offset)
        );
        assert_eq!(
            default_data.get_sunrise(),
            Sys::default().sunrise.to_offset(default_offset)
        );
        assert_eq!(
            default_data.get_sunset(),
            Sys::default().sunset.to_offset(default_offset)
        );

        let conditions = default_data.get_current_conditions();
        assert!(conditions.contains("Relative Humidity: 0%"));
        Ok(())
    }
}
