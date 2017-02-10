use std;
use std::fmt::{Display, Formatter, Result};
use json::json_chunk::JsonChunk;

pub struct JsonObject {
    radius: f64
}

impl JsonObject {
    pub fn new() -> JsonObject {
        JsonObject {
            radius: 0.1
        }
    }
}

impl JsonChunk for JsonObject {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Display for JsonObject {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", self.radius)
    }
}