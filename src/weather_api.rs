use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fmt::{self},
    hash::{Hash, Hasher},
};

use crate::Error;

#[cfg(feature = "cli")]
use reqwest::{Client, Url};

use crate::{
    apistringtype_from_display, format_string, latitude::Latitude, longitude::Longitude,
    weather_data::WeatherData, weather_forecast::WeatherForecast, ApiStringType, StringType,
};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum WeatherLocation {
    ZipCode {
        zipcode: u64,
        country_code: Option<CountryCode>,
    },
    CityName(StringType),
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

impl fmt::Display for WeatherLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZipCode {
                zipcode,
                country_code: None,
            } => {
                write!(f, "{zipcode}")
            }
            Self::ZipCode {
                zipcode,
                country_code: Some(country_code),
            } => {
                write!(f, "{zipcode} {country_code}")
            }
            Self::CityName(name) => {
                write!(f, "{name}")
            }
            Self::LatLon {
                latitude,
                longitude,
            } => {
                write!(f, "{latitude},{longitude}")
            }
        }
    }
}

impl WeatherLocation {
    #[inline]
    #[must_use]
    pub fn from_zipcode(zipcode: u64) -> Self {
        Self::ZipCode {
            zipcode,
            country_code: None,
        }
    }

    #[must_use]
    pub fn from_zipcode_country_code(zipcode: u64, country_code: CountryCode) -> Self {
        Self::ZipCode {
            zipcode,
            country_code: Some(country_code),
        }
    }

    #[must_use]
    pub fn from_zipcode_country_code_str(zipcode: u64, country_code: &str) -> Self {
        let country_code = CountryCode::for_alpha2(country_code).ok();
        Self::ZipCode {
            zipcode,
            country_code,
        }
    }

    #[must_use]
    pub fn from_city_name(city_name: &str) -> Self {
        Self::CityName(city_name.into())
    }

    #[must_use]
    pub fn from_lat_lon(latitude: Latitude, longitude: Longitude) -> Self {
        Self::LatLon {
            latitude,
            longitude,
        }
    }

    #[must_use]
    pub fn get_options(&self) -> Vec<(&'static str, ApiStringType)> {
        match self {
            Self::ZipCode {
                zipcode,
                country_code,
            } => {
                let country_code = country_code.map_or("US", |c| c.alpha2());
                let zipcode_str = apistringtype_from_display(zipcode);
                vec![("zip", zipcode_str), ("country_code", country_code.into())]
            }
            Self::CityName(city_name) => {
                let city_name = city_name.into();
                vec![("q", city_name)]
            }
            Self::LatLon {
                latitude,
                longitude,
            } => {
                let latitude_str = apistringtype_from_display(latitude);
                let longitude_str = apistringtype_from_display(longitude);
                vec![("lat", latitude_str), ("lon", longitude_str)]
            }
        }
    }

    /// Convert `WeatherLocation` to latitude/longitude
    /// # Errors
    ///
    /// Will return error if `WeatherApi::run_geo` fails
    pub async fn to_lat_lon(&self, api: &WeatherApi) -> Result<Self, Error> {
        let mut options = vec![("appid", api.api_key.clone())];
        match self {
            Self::CityName(city_name) => {
                options.push(("q", city_name.into()));
                let result: Vec<GeoLocation> = api.run_geo("direct", &options).await?;
                if let Some(loc) = result.get(0) {
                    Ok(Self::LatLon {
                        latitude: loc.lat.try_into()?,
                        longitude: loc.lon.try_into()?,
                    })
                } else {
                    Err(Error::InvalidValue("no results returned".into()))
                }
            }
            Self::ZipCode {
                zipcode,
                country_code,
            } => {
                if let Some(country_code) = country_code {
                    options.push(("zip", format_string!("{zipcode},{country_code}").into()));
                } else {
                    options.push(("zip", format_string!("{zipcode},US").into()));
                }
                let loc: GeoLocation = api.run_geo("zip", &options).await?;
                Ok(Self::LatLon {
                    latitude: loc.lat.try_into()?,
                    longitude: loc.lon.try_into()?,
                })
            }
            lat_lon @ Self::LatLon { .. } => Ok(lat_lon.clone()),
        }
    }
}

/// `WeatherApi` contains a `reqwest` Client and all the metadata required to
/// query the openweathermap.org api.
#[cfg(feature = "cli")]
#[derive(Default, Clone)]
pub struct WeatherApi {
    client: Client,
    api_key: ApiStringType,
    api_endpoint: StringType,
    api_path: StringType,
    geo_path: StringType,
}

#[cfg(feature = "cli")]
impl PartialEq for WeatherApi {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key
            && self.api_endpoint == other.api_endpoint
            && self.api_path == other.api_path
    }
}

#[cfg(feature = "cli")]
impl fmt::Debug for WeatherApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let api_key = &self.api_key;
        let api_endpoint = &self.api_endpoint;
        write!(f, "WeatherApi(key={api_key},endpoint={api_endpoint})")
    }
}

#[cfg(feature = "cli")]
impl Hash for WeatherApi {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

#[derive(Clone, Copy)]
enum WeatherCommands {
    Weather,
    Forecast,
}

impl WeatherCommands {
    fn to_str(self) -> &'static str {
        match self {
            Self::Weather => "weather",
            Self::Forecast => "forecast",
        }
    }
}

impl fmt::Display for WeatherCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GeoLocation {
    pub name: StringType,
    pub lat: f64,
    pub lon: f64,
    pub country: StringType,
    pub zip: Option<StringType>,
}

#[cfg(feature = "cli")]
impl WeatherApi {
    /// Create `WeatherApi` instance specifying `api_key`, `api_endpoint` and
    /// `api_path`
    #[must_use]
    pub fn new(api_key: &str, api_endpoint: &str, api_path: &str, geo_path: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            api_endpoint: api_endpoint.into(),
            api_path: api_path.into(),
            geo_path: geo_path.into(),
        }
    }

    #[must_use]
    pub fn with_key(self, api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_endpoint(self, api_endpoint: &str) -> Self {
        Self {
            api_endpoint: api_endpoint.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_path(self, api_path: &str) -> Self {
        Self {
            api_path: api_path.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_geo(self, geo_path: &str) -> Self {
        Self {
            geo_path: geo_path.into(),
            ..self
        }
    }

    /// Get `WeatherData` from api
    /// # Errors
    ///
    /// Will return error if `WeatherApi::run_api` fails
    pub async fn get_weather_data(&self, location: &WeatherLocation) -> Result<WeatherData, Error> {
        let options = self.get_options(location);
        self.run_api(WeatherCommands::Weather, &options).await
    }

    /// Get `WeatherForecast` from api
    /// # Errors
    ///
    /// Will return error if `WeatherApi::run_api` fails
    pub async fn get_weather_forecast(
        &self,
        location: &WeatherLocation,
    ) -> Result<WeatherForecast, Error> {
        let options = self.get_options(location);
        self.run_api(WeatherCommands::Forecast, &options).await
    }

    fn get_options(&self, location: &WeatherLocation) -> Vec<(&'static str, ApiStringType)> {
        let mut options = location.get_options();
        options.push(("appid", self.api_key.clone()));
        options
    }

    async fn run_api<T: serde::de::DeserializeOwned>(
        &self,
        command: WeatherCommands,
        options: &[(&'static str, ApiStringType)],
    ) -> Result<T, Error> {
        let api_endpoint = &self.api_endpoint;
        let api_path = &self.api_path;
        let command = format_string!("{command}");
        self._run_api(&command, options, api_endpoint, api_path)
            .await
    }

    /// Get `GeoLocation`'s from api
    /// # Errors
    ///
    /// Will return error if `WeatherApi::run_geo` fails
    pub async fn get_geo_location(
        &self,
        lat: Latitude,
        lon: Longitude,
    ) -> Result<Vec<GeoLocation>, Error> {
        let options = vec![
            ("appid", self.api_key.clone()),
            ("lat", format_string!("{lat}").into()),
            ("lon", format_string!("{lon}").into()),
        ];
        self.run_geo("reverse", &options).await
    }

    pub async fn get_zip_location(
        &self,
        zipcode: u64,
        country_code: Option<CountryCode>,
    ) -> Result<GeoLocation, Error> {
        let mut options = vec![("appid", self.api_key.clone())];
        if let Some(country_code) = &country_code {
            options.push(("zip", format_string!("{zipcode},{country_code}").into()));
        } else {
            options.push(("zip", format_string!("{zipcode},US").into()));
        }
        self.run_geo("zip", &options).await
    }

    async fn run_geo<T: serde::de::DeserializeOwned>(
        &self,
        command: &str,
        options: &[(&'static str, ApiStringType)],
    ) -> Result<T, Error> {
        let api_endpoint = &self.api_endpoint;
        let api_path = &self.geo_path;
        self._run_api(command, options, api_endpoint, api_path)
            .await
    }

    async fn _run_api<T: serde::de::DeserializeOwned>(
        &self,
        command: &str,
        options: &[(&'static str, ApiStringType)],
        api_endpoint: &str,
        api_path: &str,
    ) -> Result<T, Error> {
        let base_url = format!("https://{api_endpoint}/{api_path}{command}");
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
    use futures::future::join;
    use isocountry::CountryCode;
    use log::info;
    use std::{
        collections::hash_map::DefaultHasher,
        convert::TryInto,
        hash::{Hash, Hasher},
    };

    use crate::{weather_api::WeatherLocation, ApiStringType, Error};

    #[cfg(feature = "cli")]
    use crate::weather_api::WeatherApi;

    #[cfg(feature = "cli")]
    #[tokio::test]
    async fn test_geo_location() -> Result<(), Error> {
        let api_key = "95337ed3a8a87acae620d673fae85b11";
        let api_endpoint = "api.openweathermap.org";
        let api_path = "data/2.5/";
        let geo_path = "geo/1.0/";

        let api = WeatherApi::new(api_key, api_endpoint, api_path, geo_path);
        let loc = WeatherLocation::from_zipcode(11106);

        if let WeatherLocation::LatLon {
            latitude,
            longitude,
        } = loc.to_lat_lon(&api).await?
        {
            let lat: f64 = latitude.into();
            let lon: f64 = longitude.into();
            assert!((lat - 40.76080).abs() < 0.00001);
            assert!((lon - -73.92950).abs() < 0.00001);

            let locations = api.get_geo_location(latitude, longitude).await?;
            assert_eq!(locations.len(), 1);
            let location = &locations[0];
            assert_eq!(&location.name, "New York County");
        } else {
            assert!(false);
        }

        let loc = WeatherLocation::from_city_name("Astoria,NY,US");
        if let WeatherLocation::LatLon {
            latitude,
            longitude,
        } = loc.to_lat_lon(&api).await?
        {
            let lat: f64 = latitude.into();
            let lon: f64 = longitude.into();
            assert!((lat - 40.772014).abs() < 0.00001);
            assert!((lon - -73.93026).abs() < 0.00001);
        } else {
            assert!(false);
        }
        Ok(())
    }

    #[cfg(feature = "cli")]
    #[tokio::test]
    async fn test_process_opts() -> Result<(), Error> {
        let api_key = "95337ed3a8a87acae620d673fae85b11";
        let api_endpoint = "api.openweathermap.org";
        let api_path = "data/2.5/";
        let geo_path = "geo/1.0/";

        let api = WeatherApi::new(api_key, api_endpoint, api_path, geo_path);
        let loc = WeatherLocation::from_zipcode(11106);

        let mut hasher0 = DefaultHasher::new();
        loc.hash(&mut hasher0);
        assert_eq!(hasher0.finish(), 3871895985647742457);

        let loc = loc.to_lat_lon(&api).await?;

        let (data, forecast) =
            join(api.get_weather_data(&loc), api.get_weather_forecast(&loc)).await;
        let (data, forecast) = (data?, forecast?);
        println!("{}", data.name);
        assert!(data.name == "Queensbridge Houses");
        let timezone: i32 = forecast.city.timezone.into_inner();
        info!("{}", timezone);
        info!("{:?}", forecast);
        assert!(timezone == -18000 || timezone == -14400);
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

    #[cfg(feature = "cli")]
    #[test]
    fn test_weatherapi() -> Result<(), Error> {
        let api = WeatherApi::new("8675309", "api.openweathermap.org", "data/2.5/", "geo/1.0/");
        let api2 = WeatherApi::default()
            .with_key("8675309")
            .with_endpoint("api.openweathermap.org")
            .with_path("data/2.5/")
            .with_geo("geo/1.0/");
        assert_eq!(api, api2);

        assert_eq!(
            format!("{api:?}"),
            "WeatherApi(key=8675309,endpoint=api.openweathermap.org)".to_string()
        );

        let mut hasher0 = DefaultHasher::new();
        api.hash(&mut hasher0);
        let mut hasher1 = DefaultHasher::new();
        "WeatherApi(key=8675309,endpoint=api.openweathermap.org)"
            .to_string()
            .hash(&mut hasher1);
        info!("{:?}", api);
        assert_eq!(hasher0.finish(), hasher1.finish());

        let loc = WeatherLocation::from_zipcode_country_code(10001, CountryCode::USA);
        let opts = api.get_options(&loc);
        let expected: Vec<(&str, ApiStringType)> = vec![
            ("zip", "10001".into()),
            ("country_code", "US".into()),
            ("appid", "8675309".into()),
        ];
        assert_eq!(opts, expected);

        let loc = WeatherLocation::from_city_name("New York");
        let opts = api.get_options(&loc);
        let expected: Vec<(&str, ApiStringType)> =
            vec![("q", "New York".into()), ("appid", "8675309".into())];
        assert_eq!(opts, expected);

        let loc = WeatherLocation::from_lat_lon(41.0f64.try_into()?, 39.0f64.try_into()?);
        let opts = api.get_options(&loc);
        let expected: Vec<(&str, ApiStringType)> = vec![
            ("lat", "41.00000".into()),
            ("lon", "39.00000".into()),
            ("appid", "8675309".into()),
        ];
        assert_eq!(opts, expected);
        Ok(())
    }
}
