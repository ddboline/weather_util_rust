use anyhow::{format_err, Error};
use reqwest::{Client, Url};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::{
    latitude::Latitude, longitude::Longitude, weather_data::WeatherData,
    weather_forecast::WeatherForecast,
};

#[derive(Clone, Debug)]
pub enum WeatherLocation {
    ZipCode {
        zipcode: u64,
        country_code: Option<String>,
    },
    CityName(String),
    LatLon {
        latitude: Latitude,
        longitude: Longitude,
    },
}

impl Hash for WeatherLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self).hash(state);
    }
}

impl Default for WeatherLocation {
    fn default() -> Self {
        Self::ZipCode {
            zipcode: 10001,
            country_code: None,
        }
    }
}

impl WeatherLocation {
    pub fn from_zipcode(zipcode: u64) -> Self {
        WeatherLocation::ZipCode {
            zipcode,
            country_code: None,
        }
    }

    pub fn from_zipcode_country_code(zipcode: u64, country_code: &str) -> Self {
        let country_code = Some(country_code.to_string());
        WeatherLocation::ZipCode {
            zipcode,
            country_code,
        }
    }

    pub fn from_city_name(city_name: &str) -> Self {
        WeatherLocation::CityName(city_name.to_string())
    }

    pub fn from_lat_lon(latitude: Latitude, longitude: Longitude) -> Self {
        WeatherLocation::LatLon {
            latitude,
            longitude,
        }
    }
}

/// `WeatherApi` contains a `reqwest` Client and all the metadata required to
/// query the openweathermap.org api.
#[derive(Default, Clone)]
pub struct WeatherApi {
    client: Client,
    api_key: String,
    api_endpoint: String,
    api_path: String,
}

impl fmt::Debug for WeatherApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WeatherApi(key={},endpoint={})",
            self.api_key, self.api_endpoint
        )
    }
}

impl Hash for WeatherApi {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self).hash(state);
    }
}

impl WeatherApi {
    /// Create `WeatherApi` instance specifying `api_key`, `api_endpoint` and
    /// `api_path`
    pub fn new(api_key: &str, api_endpoint: &str, api_path: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            api_endpoint: api_endpoint.into(),
            api_path: api_path.into(),
        }
    }

    pub fn with_key(self, api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            ..self
        }
    }

    pub fn with_endpoint(self, api_endpoint: &str) -> Self {
        Self {
            api_endpoint: api_endpoint.into(),
            ..self
        }
    }

    pub fn with_path(self, api_path: &str) -> Self {
        Self {
            api_path: api_path.into(),
            ..self
        }
    }

    /// Get `WeatherData` from api
    pub async fn get_weather_data(&self, location: &WeatherLocation) -> Result<WeatherData, Error> {
        let options = self.get_options(location)?;
        self.run_api("weather", &options).await
    }

    /// Get `WeatherForecast` from api
    pub async fn get_weather_forecast(
        &self,
        location: &WeatherLocation,
    ) -> Result<WeatherForecast, Error> {
        let options = self.get_options(location)?;
        self.run_api("forecast", &options).await
    }

    fn get_options(
        &self,
        location: &WeatherLocation,
    ) -> Result<Vec<(&'static str, String)>, Error> {
        let options = match location {
            WeatherLocation::ZipCode {
                zipcode,
                country_code,
            } => {
                let country_code = country_code.clone().unwrap_or_else(|| "us".to_string());
                vec![
                    ("zip", zipcode.to_string()),
                    ("country_code", country_code),
                    ("APPID", self.api_key.to_string()),
                ]
            }
            WeatherLocation::CityName(city_name) => {
                let city_name = city_name.clone();
                vec![("q", city_name), ("APPID", self.api_key.to_string())]
            }
            WeatherLocation::LatLon {
                latitude,
                longitude,
            } => vec![
                ("lat", latitude.to_string()),
                ("lon", longitude.to_string()),
                ("APPID", self.api_key.to_string()),
            ],
        };
        Ok(options)
    }

    async fn run_api<T: serde::de::DeserializeOwned>(
        &self,
        command: &str,
        options: &[(&'static str, String)],
    ) -> Result<T, Error> {
        let base_url = format!("https://{}/{}{}", self.api_endpoint, self.api_path, command);
        let url = Url::parse_with_params(&base_url, options)?;
        let res = self.client.get(url).send().await?;
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| {
            println!("{}", text);
            e.into()
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use futures::future::join;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use crate::weather_api::{WeatherApi, WeatherLocation};

    #[tokio::test]
    async fn test_process_opts() -> Result<(), Error> {
        let api_key = "95337ed3a8a87acae620d673fae85b11";
        let api_endpoint = "api.openweathermap.org";
        let api_path = "data/2.5/";

        let api = WeatherApi::new(api_key, api_endpoint, api_path);
        let loc = WeatherLocation::from_zipcode(11106);

        let (data, forecast) =
            join(api.get_weather_data(&loc), api.get_weather_forecast(&loc)).await;
        let (data, forecast) = (data?, forecast?);
        assert!(data.name == "Astoria", format!("{:?}", data));
        let timezone: i32 = forecast.city.timezone.into();
        println!("{}", timezone);
        assert!(
            timezone == -18000 || timezone == -14400,
            format!("{:?}", forecast)
        );

        let mut hasher0 = DefaultHasher::new();
        loc.hash(&mut hasher0);
        let mut hasher1 = DefaultHasher::new();
        format!(r#"ZipCode {{ zipcode: 11106, country_code: None }}"#).hash(&mut hasher1);
        println!("{:?}", loc);
        assert_eq!(hasher0.finish(), hasher1.finish());
        Ok(())
    }
}
