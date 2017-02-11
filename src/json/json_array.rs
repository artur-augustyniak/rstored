use std::fmt::{Display, Formatter, Result};
use json::json_chunk::JsonChunk;


pub struct JsonArray {
    content: Vec<Box<JsonChunk>>
}

impl JsonArray {
    pub fn new() -> JsonArray {
        JsonArray {
            content: Vec::new()
        }
    }
}

impl JsonChunk for JsonArray {
    fn append(&mut self, node: Box<JsonChunk>) -> Option<&JsonChunk> {
        {
            let ref mut v = self.content;
            v.push(node);
        }
        Some(self)
    }
}

impl Display for JsonArray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[");
        for x in &self.content {
            write!(f, "{}", &x);
        }
        write!(f, "]")
    }
}