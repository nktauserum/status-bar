pub mod time;
pub mod battery;

pub trait Block {
    fn content(&self) -> String;
}
