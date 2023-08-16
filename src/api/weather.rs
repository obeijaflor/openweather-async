use crate::api::OpenWeatherClient;
use crate::models::*;

#[cfg(test)]
mod test {
    use crate::api::{OpenWeatherClient, Units};
    use std::env;

    #[tokio::test]
    async fn get_weather_by_city_id() -> Result<(), Box<dyn std::error::Error>> {
        let token =
            env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY environment variable");
        let client = OpenWeatherClient::new(token, Units::Metric);
        let tokyo_weather = client.get_weather_by_city_id(1850147).await?;
        // verify weather is for correct city, Tokyo, JP (city id of 1850147)
        assert_eq!(tokyo_weather.id, 1850147);
        Ok(())
    }

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
    pub async fn get_weather_by_city_id(&self, city_id: u32) -> Result<Weather, reqwest::Error> {
        let res = self
            .build_request(
                "weather",
                &vec![(String::from("id"), format!("{}", city_id))],
            )
            .await?
            .send()
            .await?;

        Ok(res.json::<Weather>().await?)
    }

    pub async fn get_weather_by_coordinates(
        &self,
        lat: f32,
        lon: f32,
    ) -> Result<Weather, reqwest::Error> {
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

        Ok(res.json::<Weather>().await?)
    }
}
