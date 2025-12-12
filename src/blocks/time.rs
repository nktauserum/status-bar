use crate::blocks::Block;
use chrono::{DateTime, Utc};

pub struct DatetimeBlock {
    format: String,
}

impl DatetimeBlock {
    pub fn new(format: &str) -> Box<Self> {
        Box::new(Self {
            format: format.to_string()
        })
    }
}

impl Block for DatetimeBlock {
    fn content(&self) -> String {
        let local_time: DateTime<Utc> = DateTime::from(Utc::now());

        format!("^b#1E1D2D^^c#D9E0EE^{time} ^c#1E1D2D^", time = local_time
            .format(self.format.as_str())
            .to_string())
    }
}