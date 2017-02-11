use std;
use std::fmt::{Display, Formatter, Result};
use json::json_chunk::JsonChunk;
use serde_json::Value;

pub struct JsonObject {
    content: Value
}

impl JsonObject {
    pub fn new() -> JsonObject {
        JsonObject {
            content: json!({})
        }
    }
}

//impl JsonChunk for JsonObject {
//    fn area(&self) -> f64 {
//        std::f64::consts::PI
//    }
//    fn append<JsonChunk: ? Sized>(&self, node: &JsonChunk) -> () {
//        match self.content {
//            Value::Object(ref array) => {
//                println!(" {}", self.content);
//            },
//            _ => panic!("not an object")
//        }
//    }
//}

impl Display for JsonObject {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.content)
    }
}