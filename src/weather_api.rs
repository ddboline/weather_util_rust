use anyhow::{format_err, Error};
use isocountry::CountryCode;
use log::error;
use reqwest::{Client, Url};
use stack_string::StackString;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::{
    latitude::Latitude, longitude::Longitude, weather_data::WeatherData,
    weather_forecast::WeatherForecast,
};

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum WeatherLocation {
    ZipCode {
        zipcode: u64,
        country_code: Option<CountryCode>,
    },
    CityName(StackString),
    LatLon {
        latitude: Latitude,
        longitude: Longitude,
    },
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
        let country_code = CountryCode::for_alpha2(country_code).ok();
        WeatherLocation::ZipCode {
            zipcode,
            country_code,
        }
    }

    pub fn from_city_name(city_name: &str) -> Self {
        WeatherLocation::CityName(city_name.into())
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
    api_key: StackString,
    api_endpoint: StackString,
    api_path: StackString,
}

impl PartialEq for WeatherApi {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key
            && self.api_endpoint == other.api_endpoint
            && self.api_path == other.api_path
    }
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
    ) -> Result<Vec<(&'static str, StackString)>, Error> {
        let options: Vec<(&'static str, StackString)> = match location {
            WeatherLocation::ZipCode {
                zipcode,
                country_code,
            } => {
                let country_code = country_code.map_or("US" ,|c| c.alpha2());
                vec![
                    ("zip", zipcode.to_string().into()),
                    ("country_code", country_code.into()),
                    ("APPID", self.api_key.clone()),
                ]
            }
            WeatherLocation::CityName(city_name) => {
                let city_name = city_name.clone();
                vec![("q", city_name), ("APPID", self.api_key.clone())]
            }
            WeatherLocation::LatLon {
                latitude,
                longitude,
            } => vec![
                ("lat", latitude.to_string().into()),
                ("lon", longitude.to_string().into()),
                ("APPID", self.api_key.clone()),
            ],
        };
        Ok(options)
    }

    async fn run_api<T: serde::de::DeserializeOwned>(
        &self,
        command: &str,
        options: &[(&'static str, StackString)],
    ) -> Result<T, Error> {
        let base_url = format!("https://{}/{}{}", self.api_endpoint, self.api_path, command);
        let url = Url::parse_with_params(&base_url, options)?;
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use futures::future::join;
    use stack_string::StackString;
    use std::{
        collections::hash_map::DefaultHasher,
        convert::TryInto,
        hash::{Hash, Hasher},
    };

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
        println!("{:?}", data);
        assert!(data.name == "Astoria");
        let timezone: i32 = forecast.city.timezone.into();
        println!("{}", timezone);
        println!("{:?}", forecast);
        assert!(timezone == -18000 || timezone == -14400);

        let mut hasher0 = DefaultHasher::new();
        loc.hash(&mut hasher0);
        assert_eq!(hasher0.finish(), 3871895985647742457);
        Ok(())
    }

    #[test]
    fn test_weatherlocation_default() -> Result<(), Error> {
        assert_eq!(
            WeatherLocation::default(),
            WeatherLocation::from_zipcode(10001)
        );
        Ok(())
    }

    #[test]
    fn test_weatherapi() -> Result<(), Error> {
        let api = WeatherApi::new("8675309", "api.openweathermap.org", "data/2.5/");
        let api2 = WeatherApi::default()
            .with_key("8675309")
            .with_endpoint("api.openweathermap.org")
            .with_path("data/2.5/");
        assert_eq!(api, api2);

        assert_eq!(
            format!("{:?}", api),
            "WeatherApi(key=8675309,endpoint=api.openweathermap.org)".to_string()
        );

        let mut hasher0 = DefaultHasher::new();
        api.hash(&mut hasher0);
        let mut hasher1 = DefaultHasher::new();
        "WeatherApi(key=8675309,endpoint=api.openweathermap.org)"
            .to_string()
            .hash(&mut hasher1);
        println!("{:?}", api);
        assert_eq!(hasher0.finish(), hasher1.finish());

        let loc = WeatherLocation::from_zipcode_country_code(10001, "US");
        let opts = api.get_options(&loc)?;
        let expected: Vec<(&str, StackString)> = vec![
            ("zip", "10001".into()),
            ("country_code", "US".into()),
            ("APPID", "8675309".into()),
        ];
        assert_eq!(opts, expected);

        let loc = WeatherLocation::from_city_name("New York");
        let opts = api.get_options(&loc)?;
        let expected: Vec<(&str, StackString)> =
            vec![("q", "New York".into()), ("APPID", "8675309".into())];
        assert_eq!(opts, expected);

        let loc = WeatherLocation::from_lat_lon(41.0f64.try_into()?, 39.0f64.try_into()?);
        let opts = api.get_options(&loc)?;
        let expected: Vec<(&str, StackString)> = vec![
            ("lat", "41".into()),
            ("lon", "39".into()),
            ("APPID", "8675309".into()),
        ];
        assert_eq!(opts, expected);
        Ok(())
    }
}
