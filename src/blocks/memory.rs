use crate::blocks::{Block, LastUpdated};
use std::process::Command;

pub struct MemoryBlock {
    last: LastUpdated,
}

impl MemoryBlock {
    pub fn new(interval: u64) -> Box<Self> {
        Box::new(Self {
            last: LastUpdated::new(interval),
        })
    }
}

impl Block for MemoryBlock {
    fn content(&self) -> String {
       let cmd_str = "free -h | awk '/^Mem/ { print $3 }' | sed 's/i//g'";

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output();

        if let Ok(bytes) = output {
            return format!("^c#D9E0EE^^b#1E1D2D^Mem {load}^c#1E1D2D^", load = String::from_utf8(bytes.stdout).expect("Некорректная UTF-8 последовательность").trim_end_matches('\n'));
        } else if let Err(e) = output {
            eprintln!("[ERROR]: CPUBlock error: {e}");
        }
        self.last.get_last_result()
    }
}