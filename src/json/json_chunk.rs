use std::fmt::Display;

pub trait JsonChunk: Display {
    fn append(&mut self, node: Box<JsonChunk>) -> ();
}