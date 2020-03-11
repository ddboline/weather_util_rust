use anyhow::{format_err, Error};
use reqwest::{Client, Url};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::latitude::Latitude;
use crate::longitude::Longitude;
use crate::weather_data::WeatherData;
use crate::weather_forecast::WeatherForecast;

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

/// `WeatherApi` contains a `reqwest` Client and all the metadata required to query the openweathermap.org api.
#[derive(Default, Clone)]
pub struct WeatherApi {
    client: Client,
    api_key: String,
    api_endpoint: String,
    api_path: String,
    location: WeatherLocation,
}

impl fmt::Debug for WeatherApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}{}{}",
            self.location, self.api_key, self.api_endpoint
        )
    }
}

impl Hash for WeatherApi {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self).hash(state);
    }
}

impl WeatherApi {
    /// Create `WeatherApi` instance specifying `api_key`, `api_endpoint` and `api_path`
    pub fn new(api_key: &str, api_endpoint: &str, api_path: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            api_endpoint: api_endpoint.into(),
            api_path: api_path.into(),
            ..Self::default()
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

    pub fn with_zipcode(self, zipcode: u64) -> Self {
        Self {
            location: WeatherLocation::ZipCode {
                zipcode,
                country_code: None,
            },
            ..self
        }
    }

    pub fn with_zipcode_country_code(self, zipcode: u64, country_code: &str) -> Self {
        let country_code = Some(country_code.to_string());
        Self {
            location: WeatherLocation::ZipCode {
                zipcode,
                country_code,
            },
            ..self
        }
    }

    pub fn with_city_name(self, city_name: &str) -> Self {
        Self {
            location: WeatherLocation::CityName(city_name.to_string()),
            ..self
        }
    }

    pub fn with_lat_lon(self, latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            location: WeatherLocation::LatLon {
                latitude,
                longitude,
            },
            ..self
        }
    }

    /// Get `WeatherData` from api
    pub async fn get_weather_data(&self) -> Result<WeatherData, Error> {
        let options = self.get_options()?;
        self.run_api("weather", &options).await
    }

    /// Get `WeatherForecast` from api
    pub async fn get_weather_forecast(&self) -> Result<WeatherForecast, Error> {
        let options = self.get_options()?;
        self.run_api("forecast", &options).await
    }

    fn get_options(&self) -> Result<Vec<(&'static str, String)>, Error> {
        let options = match &self.location {
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

    use crate::weather_api::WeatherApi;

    #[tokio::test]
    async fn test_process_opts() -> Result<(), Error> {
        let api_key = "95337ed3a8a87acae620d673fae85b11";
        let api_endpoint = "api.openweathermap.org";
        let api_path = "data/2.5/";

        let api = WeatherApi::new(api_key, api_endpoint, api_path).with_zipcode(11106);

        let (data, forecast) = join(api.get_weather_data(), api.get_weather_forecast()).await;
        let (data, forecast) = (data?, forecast?);
        assert!(data.name == "Astoria", format!("{:?}", data));
        println!("{}", forecast.city.timezone);
        assert!(
            forecast.city.timezone == -18000 || forecast.city.timezone == -14400,
            format!("{:?}", forecast)
        );
        Ok(())
    }
}
