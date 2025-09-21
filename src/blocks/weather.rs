use crate::blocks::{Block, LastUpdated};
use openweather_sdk::responses::CurrentResponse;
use reqwest::blocking::Client;


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