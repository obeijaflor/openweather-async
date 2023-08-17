use serde::{Deserialize, Serialize};

/// Represents the accepted units for use with OpenWeatherMap.org.  Units are documented at the
/// (confusingly named) URL <https://openweathermap.org/current#data>.
pub enum Units {
    /// Standard is the default.  Temperatures are presented in Kelvin, e.g. 296 K.
    Standard,
    /// Metric provides temperature in Celsius, e.g. 23°C.
    Metric,
    /// Imperial (a/k/a U.S. customary) provides temperatures in Fahrenheit, e.g. 73°F.
    Imperial,
}

/// Represents a coordinate location (latitude, longitude) on Earth.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Coordinate {
    /// Latitude, as a decimal number.
    pub lat: Option<f32>,
    /// Longitude, as a decimal number.
    pub lon: Option<f32>,
}

/// Represents a weather condition and shows an overall description of the current weather, along
/// with an ID for the relevant representative icon.
///
/// See <https://openweathermap.org/weather-conditions> for more info.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WeatherCondition {
    /// Weather condition ID.
    pub id: u32,
    /// Not well defined in spec but seems to be string.  Is this a "weather title"?
    pub main: String,
    /// Description of weather.
    pub description: String,
    /// Weather icon identifier.
    #[serde(rename = "icon")]
    pub icon_id: String,
}

/// Provides "main" weather information most people are probably looking for, like temperatures,
/// pressure, humidity, etc.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MainInformation {
    /// Current temperature, in the specified units.
    pub temp: f32,
    /// "Feels like" temperature, in the specified units.
    pub feels_like: f32,
    /// Minimum temperature for the observed area, in the specified units.
    pub temp_min: f32,
    /// Maximum temperature for the observed area, in the specified units.
    pub temp_max: f32,
    /// Humidity, as a percentage.
    pub humidity: f32,
    /// Atmospheric pressure at sea level, in hectopascals (hPa).
    pub pressure: f32,
    /// Atmospheric pressure at sea level, in hectopascals (hPa).
    pub sea_level: Option<f32>,
    /// Atmospheric pressure at ground level, in hectopascals (hPa).
    pub grnd_level: Option<f32>,
    /// An internal parameter.
    pub temp_kf: Option<f32>,
}

//// Provides wind information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WindInformation {
    /// Wind speed in m/s (standard/metric) or mph (imperial).
    pub speed: f32,
    /// Wind direction, in degrees.
    pub deg: f32,
    /// Wind gust in m/s (standard/metric) or mph (imperial).
    pub gust: Option<f32>,
}

/// If present, provides rainfall totals.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RainfallTotals {
    /// Rainfall total over the last hour.
    #[serde(rename = "1h")]
    pub last_1h: Option<f32>,
    /// Rainfall total over the last 3 hours.
    #[serde(rename = "3h")]
    pub last_3h: Option<f32>,
}

/// If present, provides snowfall totals.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SnowfallTotals {
    /// Snowfall total over the last hour.
    #[serde(rename = "1h")]
    pub last_1h: Option<f32>,
    /// Snowfall total over the last 3 hours.
    #[serde(rename = "3h")]
    pub last_3h: Option<f32>,
}

/// Represents cloudiness statistics.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Cloudiness {
    /// Cloudiness percentage.
    #[serde(rename = "all")]
    pub percentage: Option<u32>,
}

/// Payload from the API 2.5 /weather endpoint.
///
/// Provides current weather data as defined here: <https://openweathermap.org/current>.
/// The spec is very vague and could use some work.  Types are not well-defined.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CurrentWeatherPayload {
    /// Location coordinates for the provided weather information.
    pub coord: Coordinate,
    /// Represents current high-level weather conditions.
    #[serde(rename = "weather")]
    pub weather_conditions: Option<Vec<WeatherCondition>>,
    /// An internal parameter, which seems to be string of unknown purpose.
    pub base: Option<String>,
    /// Provides "main" info like current temp, pressure, humidity, etc.
    pub main: MainInformation,
    /// Visibility in meters.   Maximum value is 10000 meters a/k/a 10 km.
    pub visibility: Option<u32>,
    /// Wind information.
    pub wind: WindInformation,
    /// Rainfall totals.
    #[serde(rename = "rain")]
    pub rainfall: Option<RainfallTotals>,
    /// Snowfall totals.
    #[serde(rename = "snow")]
    pub snowfall: Option<SnowfallTotals>,
    /// Cloudiness information.
    #[serde(rename = "clouds")]
    pub cloudiness: Cloudiness,
    /// Data calculation time, as a UNIX timestamp, relative to UTC.
    #[serde(rename = "dt")]
    pub dt_unix_timestamp: u32,
    /// Bad name.  Some internal "system" information, but also sunset/sunrise info.
    pub sys: CurrentWeatherSys,
    /// Timezone shift relative to UTC.
    pub timezone: Option<i32>,
    /// The city ID.
    pub id: i32,
    /// The city name.
    pub name: String,
    /// An internal parameter - some sort of integer, probably a status code.
    pub cod: Option<u32>,
}

/// Provides some internal information and sunrise/sunset info as a UNIX timestamp.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CurrentWeatherSys {
    /// type - internal parameter
    #[serde(rename = "type")]
    pub message_type: Option<u32>,
    /// id - internal parameter
    pub id: Option<u32>,
    /// Country code.
    pub country: String,
    #[serde(rename = "sunrise")]
    /// Sunrise time, as a UNIC timestamp relative to UTC.
    pub sunrise_timestamp: Option<u32>,
    /// Sunset time, as a UNIC timestamp relative to UTC.
    #[serde(rename = "sunset")]
    pub sunset_timestamp: Option<u32>,
}

/// Payload from the API 2.5 /forecast endpoint.
///
/// Provides current weather data as defined here: <https://openweathermap.org/forecast5>.
/// The spec is very vague and could use some work.  Types are not well-defined.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Forecast5Payload {
    /// An internal parameter - some sort of integer, probably a status code.
    pub cod: Option<u32>,
    /// An internal parameter.
    pub message: String,
    /// The number of timestamps returned in the API response.
    pub cnt: usize,
    /// A list of weather forecasts for the next 5 days, with samples every 3 hours.
    #[serde(rename = "list")]
    pub forecast_list: Forecast5DaysList,
    /// Another bad name.  Some city information, but also sunset/sunrise info.
    #[serde(rename = "city")]
    pub city_info: Forecast5CityInfo,
}

/// Provides some city information along with sunset/sunrise timestamps.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Forecast5CityInfo {
    /// The city ID.
    pub id: i32,
    /// The city name.
    pub name: String,
    /// Location coordinates for the provided weather information.
    pub coord: Coordinate,
    /// Country code.
    pub country: String,
    /// Population of the city.  (Why is this part of the API?  Who knows.)
    pub population: usize,
    /// Timezone shift relative to UTC.
    pub timezone: Option<i32>,
    /// Sunrise time, as a UNIC timestamp relative to UTC.
    #[serde(rename = "sunrise")]
    pub sunrise_timestamp: Option<u32>,
    /// Sunset time, as a UNIC timestamp relative to UTC.
    #[serde(rename = "sunset")]
    pub sunset_timestamp: Option<u32>,
}

/// A list of weather forecasts for the next 5 days, with samples every 3 hours.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Forecast5DaysList {
    /// Data calculation time, as a UNIX timestamp, relative to UTC.
    #[serde(rename = "dt")]
    pub dt_unix_timestamp: u32,
    /// Data calculation time, in ISO 8601 format, relative to UTC.
    #[serde(rename = "dt_txt")]
    pub dt_iso8601_str: String,
    /// Provides "main" info like current temp, pressure, humidity, etc.
    pub main: MainInformation,
    /// Represents current high-level weather conditions.
    #[serde(rename = "weather")]
    pub weather_conditions: Option<Vec<WeatherCondition>>,
    /// Cloudiness information.
    #[serde(rename = "clouds")]
    pub cloudiness: Cloudiness,
    /// Wind information.
    pub wind: WindInformation,
    /// Average visibility, in meters.   Maximum value is 10000 meters a/k/a 10 km.
    pub visibility: Option<u32>,
    /// Probability of precipitation. The values of the parameter vary between 0 and 1, where 0 is
    /// equal to 0% and 1 is equal to 100%
    #[serde(rename = "pop")]
    pub probability_of_precipitation: f32,
    /// Rainfall totals.
    #[serde(rename = "rain")]
    pub rainfall: Option<RainfallTotals>,
    /// Snowfall totals.
    #[serde(rename = "snow")]
    pub snowfall: Option<SnowfallTotals>,
    /// Horrible name.  Whatever "sys" means in the API documentation.
    pub sys: Forecast5Sys,
}

/// Horrible name.  Whatever "sys" means in the API documentation.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Forecast5Sys {
    /// Part of the day (n - night, d - day)
    #[serde(rename = "pod")]
    pub part_of_day: String,
}
