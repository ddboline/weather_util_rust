use anyhow::{format_err, Error};
use reqwest::{Client, Url};

use crate::latitude::Latitude;
use crate::longitude::Longitude;
use crate::weather_data::WeatherData;
use crate::weather_forecast::WeatherForecast;

/// `WeatherApi` contains a `reqwest` Client and all the metadata required to query the openweathermap.org api.
#[derive(Default, Clone)]
pub struct WeatherApi {
    client: Client,
    api_key: String,
    api_endpoint: String,
    zipcode: Option<u64>,
    country_code: Option<String>,
    city_name: Option<String>,
    lat_lon: Option<(Latitude, Longitude)>,
}

impl WeatherApi {
    /// Create `WeatherApi` instance specifying api_key and api_endpoint
    pub fn new(api_key: &str, api_endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            api_endpoint: api_endpoint.into(),
            ..Self::default()
        }
    }

    pub fn with_zipcode(self, zipcode: u64) -> Self {
        Self {
            zipcode: Some(zipcode),
            ..self
        }
    }

    pub fn with_country_code(self, country_code: &str) -> Self {
        Self {
            country_code: Some(country_code.into()),
            ..self
        }
    }

    pub fn with_city_name(self, city_name: &str) -> Self {
        Self {
            city_name: Some(city_name.into()),
            ..self
        }
    }

    pub fn with_lat_lon(self, lat: Latitude, lon: Longitude) -> Self {
        Self {
            lat_lon: Some((lat, lon)),
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
        let options = if let Some(zipcode) = self.zipcode {
            let country_code = self
                .country_code
                .clone()
                .unwrap_or_else(|| "us".to_string());
            vec![
                ("zip", zipcode.to_string()),
                ("country_code", country_code),
                ("APPID", self.api_key.to_string()),
            ]
        } else if let Some(city_name) = self.city_name.clone() {
            vec![("q", city_name), ("APPID", self.api_key.to_string())]
        } else if let Some((lat, lon)) = self.lat_lon {
            vec![
                ("lat", lat.to_string()),
                ("lon", lon.to_string()),
                ("APPID", self.api_key.to_string()),
            ]
        } else {
            return Err(format_err!(
                "\n\nERROR: You must specify at least one option"
            ));
        };
        Ok(options)
    }

    async fn run_api<T: serde::de::DeserializeOwned>(
        &self,
        command: &str,
        options: &[(&'static str, String)],
    ) -> Result<T, Error> {
        let base_url = format!("https://{}/data/2.5/{}", self.api_endpoint, command);
        let url = Url::parse_with_params(&base_url, options)?;
        let res = self.client.get(url).send().await?;
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| {
            println!("{}", text);
            e.into()
        })
    }

    pub fn weather_api_hash(&self) -> String {
        format!(
            "zipcode:{:?},country_code:{:?},city_name:{:?},lat_lon:{:?}",
            self.zipcode, self.country_code, self.city_name, self.lat_lon
        )
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

        let api = WeatherApi::new(api_key, api_endpoint).with_zipcode(11106);

        let (data, forecast) = join(api.get_weather_data(), api.get_weather_forecast()).await;
        let (data, forecast) = (data?, forecast?);
        assert!(data.name == "Astoria", format!("{:?}", data));
        assert!(forecast.city.timezone == -18000, format!("{:?}", forecast));
        Ok(())
    }
}
