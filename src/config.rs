use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub interval_all: u64,
    pub datetime: DateTimeConfig,
    pub battery: BatteryConfig,
    pub weather: WeatherConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTimeConfig {
    pub offset: i32,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatteryConfig {
    pub interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub place: WeatherPlace,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherPlace {
    pub lat: String,
    #[serde(rename = "long")]
    pub long: String,
}


impl Config {
    // Load читает конфигурационный файл по переданному пути и паникует, если в процессе происходит ошибка. Возвращает экземпляр Config.
    pub fn load(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap();

        serde_json::from_str(&contents).unwrap()
    }
}