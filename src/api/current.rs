use crate::api::OpenWeatherClient;
use crate::models::CurrentWeatherPayload;

#[cfg(test)]
mod test {
    use crate::api::OpenWeatherClient;
    use crate::models::Units;
    use std::env;

    #[tokio::test]
    async fn get_weather_by_coordinates() -> Result<(), Box<dyn std::error::Error>> {
        let token =
            env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY environment variable");
        let client = OpenWeatherClient::new(token, Units::Metric);
        let sf_weather = client
            .get_weather_by_coordinates(37.766602, -122.45108)
            .await?;
        // verify weather is for correct city, San Francisco, CA, US (city id of 5391997)
        assert_eq!(sf_weather.id, 5391997);
        Ok(())
    }
}

impl OpenWeatherClient {
    pub async fn get_weather_by_coordinates(
        &self,
        lat: f32,
        lon: f32,
    ) -> Result<CurrentWeatherPayload, reqwest::Error> {
        let res = self
            .build_request(
                "weather",
                &vec![
                    (String::from("lat"), format!("{}", lat)),
                    (String::from("lon"), format!("{}", lon)),
                ],
            )
            .await?
            .send()
            .await?;

        res.json::<CurrentWeatherPayload>().await
    }
}
