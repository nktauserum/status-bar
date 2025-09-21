mod blocks;

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use crate::blocks::{
    Block,
    
    time::DatetimeBlock
};

struct Bar {
    update_interval: u64,
    blocks: Vec<Box<dyn Block>>,
}

impl Bar {
    fn new(update_interval: u64, blocks: Vec<Box<dyn Block>>) -> Self {
        Self {
            update_interval,
            blocks
        }
    }

    fn update(&self, content: String) {
        let update_cmd = Command::new("xsetroot")
            .arg("-name")
            .arg(&content)
            .spawn();

        if let Ok(_) = update_cmd {
            println!("[INFO]: statusbar updated with content \"{content}\"");
        } else if let Err(e) = update_cmd {
            eprintln!("[ERROR]: update error: {e}");
        }
    }

    fn run(&self) {
        loop {
            let mut upd_str: Vec<String> = vec![];

            for block in &self.blocks {
                upd_str.push(block.content())
            }

            self.update(upd_str.join(" | "));

            sleep(Duration::from_millis(self.update_interval));
        }
    }
}

fn main() {
    let bar = Bar::new(500, vec![
        DatetimeBlock::new(3, "%a, %d %b %H:%M")
    ]);

    bar.run();
}
