use crate::api::OpenWeatherClient;
use crate::models::Forecast5Payload;

impl OpenWeatherClient {
    pub async fn get_forecast5_by_coordinates(
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
