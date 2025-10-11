use std::process::Command;
use crate::blocks::{Block, LastUpdated};

pub struct CPUBlock {
    last: LastUpdated,
}

impl CPUBlock {
    pub fn new(interval: u64) -> Box<Self> {
        Box::new(Self {
            last: LastUpdated::new(interval),
        })
    }
}

impl Block for CPUBlock {
    fn content(&self) -> String {
        let cmd = Command::new("grep")
            .arg("-o")
            .arg("^[^ ]*")
            .arg("/proc/loadavg")
            .output();

        if let Ok(bytes) = cmd {
            return format!("CPU: {load}", load = String::from_utf8(bytes.stdout).expect("Некорректная UTF-8 последовательность").trim_end_matches('\n'));
        } else if let Err(e) = cmd {
            eprintln!("[ERROR]: CPUBlock error: {e}");
        }
        self.last.get_last_result()
    }
}