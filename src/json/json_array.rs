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
    fn append(&mut self, node: Box<JsonChunk>) -> () {
        self.content.push(node);
    }
}

impl Display for JsonArray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[");
        let mut range = self.content.iter();
        match range.next() {
            Some(token) => { write!(f, "{}", &token); }
            _ => (),
        }
        for x in range {
            write!(f, ",{}", &x);
        }
        write!(f, "]")
    }
}