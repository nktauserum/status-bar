use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::blocks::Block;
use battery::Manager;

pub struct BatteryBlock {
    interval: u64,
    last_update: Arc<Mutex<Instant>>,
    last_result: Arc<Mutex<u32>>,
}

impl BatteryBlock {
    pub fn new(interval: u64) -> Box<Self> {
        Box::new(
            Self {
                interval,
                last_update: Arc::new(Mutex::new(Instant::now())),
                last_result: Arc::new(Mutex::new(0)),
            }
        )
    }

    fn need_to_update(&self) -> bool {
        let elapsed = self.last_update.lock().expect("locking last_update").elapsed();

        elapsed >= Duration::from_millis(self.interval)
    }

    fn build(&self) -> Result<u32, battery::Error> {
        if !self.need_to_update() {
            return Ok(
                *self.last_result.lock().expect("locking last_result").deref()
            );
        }

        *self.last_update.lock().expect("locking last_update") = Instant::now();

        let mn = Manager::new()?;
        let batts = mn.batteries()?;

        Ok({
                let mut result: u32 = 0;
                for battery in batts {
                    let state = battery?.state_of_charge().value;
                    result = state as u32 * 100;
                }

                *self.last_result.lock().expect("locking last_result") = result;

                result
            }
        )
    }
}

impl Block for BatteryBlock {
    fn content(&self) -> String {
        let proc = self.build().unwrap_or_else(|err| {
            eprintln!("[ERROR]: battery update: {err}");
            *self.last_result.lock().expect("locking last_update").deref()
        });

        format!("{proc}%")
    }
}