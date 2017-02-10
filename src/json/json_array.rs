use std::f64::consts::PI;
use std::fmt::{Display, Formatter, Result};
use json::json_chunk::JsonChunk;

pub struct JsonArray {
    radius: f64
}

impl JsonArray {
    pub fn new() -> JsonArray {
        JsonArray {
            radius: 0.1
        }
    }
}

impl JsonChunk for JsonArray {
    fn area(&self) -> f64 {
        PI * (self.radius * self.radius)
    }
}

impl Display for JsonArray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", self.radius)
    }
}