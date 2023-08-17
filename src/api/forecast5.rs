use crate::api::OpenWeatherClient;
use crate::models::Forecast5Payload;

#[cfg(test)]
mod test {
    use crate::api::OpenWeatherClient;
    use crate::models::Units;
    use std::env;

    #[tokio::test]
    async fn get_forecast5() -> Result<(), Box<dyn std::error::Error>> {
        let token =
            env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY environment variable");
        let client = OpenWeatherClient::new(token, Units::Metric);
        let sf_weather = client.get_forecast5(37.766602, -122.45108, Some(1)).await?;
        // verify weather is for correct city, San Francisco, CA, US (city id of 5391997)
        assert_eq!(sf_weather.city_info.city_id, 5391997);
        assert_eq!(sf_weather.city_info.city_name, "San Francisco County");
        Ok(())
    }
}

impl OpenWeatherClient {
    pub async fn get_forecast5(
        &self,
        lat: f32,
        lon: f32,
        cnt: Option<usize>,
    ) -> Result<Forecast5Payload, reqwest::Error> {
        let mut params = vec![
            (String::from("lat"), format!("{}", lat)),
            (String::from("lon"), format!("{}", lon)),
        ];
        if let Some(count) = cnt {
            params.push((String::from("cnt"), format!("{}", count)));
        }
        let res = self
            .build_request("forecast", &params)
            .await?
            .send()
            .await?;

        res.json::<Forecast5Payload>().await
    }
}
