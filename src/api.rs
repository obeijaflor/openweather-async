pub use crate::models::*;

pub mod current;
pub mod forecast5;

impl Units {
    pub fn value(&self) -> &str {
        match *self {
            Units::Standard => "standard",
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    }
}

/// A client for interacting with the OpenWeatherMap.org API.
pub struct OpenWeatherClient {
    api_key: String,
    client: reqwest::Client,
    pub units: Units,
}

impl OpenWeatherClient {
    pub const API_BASE_URL: &'static str = "https://api.openweathermap.org/data/2.5/";

    /// Constructs a new `OpenWeatherClient` with the provided API key and preferred unit
    /// information.
    pub fn new(api_key: String, units: Units) -> OpenWeatherClient {
        OpenWeatherClient {
            api_key,
            client: reqwest::Client::new(),
            units,
        }
    }

    /// Provides a `reqwest::RequestBuilder` targeting the provided endpoint.  Adds `extra_params`
    /// to a query string pre-populated with the API token and preferred unit information.
    ///
    /// The provided request should be sent at the caller's discretion, and could be reused.
    async fn build_request(
        &self,
        endpoint: &str,
        extra_params: &Vec<(String, String)>,
    ) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let addr = format!("{}{}", &Self::API_BASE_URL, &endpoint);

        let mut params = vec![
            (String::from("appid"), self.api_key.clone()),
            (String::from("units"), String::from(self.units.value())),
        ];
        params.extend_from_slice(extra_params.as_slice());

        let unsent_request = self.client.get(addr).query(&params);
        Ok(unsent_request)
    }
}
