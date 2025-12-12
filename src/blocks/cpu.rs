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
            let result = format!("^c#D9E0EE^^b#1E1D2D^CPU {load}^c#1E1D2D^", load = String::from_utf8(bytes.stdout).expect("Некорректная UTF-8 последовательность").trim_end_matches('\n'));
            self.last.set_last_result(result.clone());
            return result;
        } else if let Err(e) = cmd {
            eprintln!("[ERROR]: CPUBlock error: {e}");
        }
        format!("^c#FFCC00^[⚠]^c#1E1D2D^ {}", self.last.get_last_result())
    }
}