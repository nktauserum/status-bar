use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub mod time;
pub mod battery;
pub mod weather;
pub mod cpu;

pub trait Block {
    fn content(&self) -> String;
}

pub struct LastUpdated {
    interval: u64,
    last_update: Arc<Mutex<Instant>>,
    last_result: Arc<Mutex<String>>,
}

impl LastUpdated {
    pub fn new(interval: u64) -> Self {
        let last = Instant::now() - Duration::from_millis(interval + 1);
        Self {
            interval,
            last_update: Arc::new(Mutex::new(last)),
            last_result: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn needs_update(&self) -> bool {
        let elapsed = self.last_update.lock().expect("locking last_update").elapsed();

        elapsed >= Duration::from_millis(self.interval)
    }

    pub fn get_last_result(&self) -> String {
        let guard = self.last_result.lock().expect("locking last_result");
        guard.clone()
    }

    pub fn set_last_result(&self, value: String) {
        {
            let mut guard = self.last_result.lock().expect("locking last_result");
            *guard = value;
        }
        let mut t_guard = self.last_update.lock().expect("locking last_update");
        *t_guard = Instant::now();
    }
}