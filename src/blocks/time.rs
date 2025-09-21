use crate::blocks::Block;
use chrono::{DateTime, FixedOffset, Utc};

pub struct DatetimeBlock {
    timezone_offset: i32,
    format: String,
}

impl DatetimeBlock {
    pub fn new(timezone_offset: i32, format: &str) -> Box<Self> {
        Box::new(Self {
            timezone_offset,
            format: format.to_string()
        })
    }
}

impl Block for DatetimeBlock {
    fn content(&self) -> String {
        let utc_now: DateTime<Utc> = Utc::now();
        let tz = FixedOffset::east_opt(self.timezone_offset * 3600);
        let local_time = {
            if let Some(timezone) = tz {
                utc_now.with_timezone(&timezone)
            } else {
                DateTime::from(utc_now)
            }
        };

        local_time
            .format(self.format.as_str())
            .to_string()
    }
}