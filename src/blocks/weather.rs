use crate::blocks::{Block, LastUpdated};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub struct WeatherBlock {
    last: LastUpdated,
    key: String,
    lat: f64,
    lon: f64,
    client: Client,
}

impl WeatherBlock {
    pub fn new(interval: u64, api_key: String, lat: f64, lon: f64) -> Box<Self> {
        Box::new(
            Self {
                last: LastUpdated::new(interval),
                key: api_key,
                client: Client::new(),
                lat, lon,
            }
        )
    }

    pub fn build(&self) -> Result<CurrentResponse, Box<dyn std::error::Error>> {
        println!("[DEBUG]: building weather response");

        let response = self.client.get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&lang={}&appid={}",
            self.lat,
            self.lon,
            "metric",
            "ru",
            self.key));

        let resp = response.send()?.text()?;
        println!("[DEBUG]: {r}", r = resp.clone());

        Ok(serde_json::from_str(resp.as_str())?)
    }

}

impl Block for WeatherBlock {
    fn content(&self) -> String {
        if !self.last.needs_update() {
            return self.last.get_last_result();
        }

        let res = self.build()
            .map(|resp| {
                let desc = &resp.weather[0].description;
                let upper_first = match desc.chars().next() {
                    Some(first) => format!("{}{}", first.to_uppercase(), &desc[first.len_utf8()..]),
                    None => String::new(),
                };

                let result = format!("{upper_first} {:.0}°C", resp.main.temp);
                self.last.set_last_result(result.clone());
                result
            }).unwrap_or_else(|err| {
                eprintln!("[ERROR]: weather update: {err}");
                self.last.get_last_result()
            });

        res
    }
}

// API response implementations: https://github.com/jt-rose/openweather_sdk
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sys {
    #[serde(alias = "type")]
    pub country: String,
    pub sunrise: usize,
    pub sunset: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u64,
    pub humidity: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: usize,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: usize,
    pub sys: Sys,
    pub timezone: i64,
    pub id: usize,
    pub name: String,
    pub cod: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clouds {
    pub all: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weather {
    pub id: u64,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: Option<f64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64
}