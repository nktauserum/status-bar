use crate::blocks::{Block, LastUpdated};
use battery::Manager;

pub struct BatteryBlock {
    last: LastUpdated
}

impl BatteryBlock {
    pub fn new(interval: u64) -> Box<Self> {
        Box::new(
            Self {
                last: LastUpdated::new(interval)
            }
        )
    }

    pub fn build(&self) -> Result<String, battery::Error> {
        let mn = Manager::new()?;
        let batts = mn.batteries()?;

        let mut proc: u32 = 0;
        // Для одной батареи, как в моём случае, это корректно.
        for battery in batts {
            let state = battery?.state_of_charge().value;
            proc = (state as f32 * 100f32) as u32;
        }

        let res = format!("{proc}%");
        self.last.set_last_result(res.clone());

        Ok(res)
    }

}

impl Block for BatteryBlock {
    fn content(&self) -> String {
        if !self.last.needs_update() {
            return self.last.get_last_result();
        }

        let res = self.build().unwrap_or_else(|err| {
            eprintln!("[ERROR]: battery update: {err}");
            self.last.get_last_result()
        });

        res
    }
}