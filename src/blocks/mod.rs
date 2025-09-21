pub mod time;

pub trait Block {
    fn content(&self) -> String;
}
