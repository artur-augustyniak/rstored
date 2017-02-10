use std::fmt::Display;
pub trait JsonChunk: Display {
    fn area(&self) -> f64;
}