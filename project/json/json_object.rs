use std::fmt::{Display, Formatter, Result};
use json::json_chunk::JsonChunk;


enum JsonFieldValue {
    Null,
    Bool(bool),
    s64,
    String(String),
    JsonArray,
    JsonObject,
}

struct JsonField {
    name: String,
    value: JsonFieldValue
}


pub struct JsonObject {
    content: Vec<Box<JsonField>>
}

//impl JsonObject {
//    pub fn new() -> JsonObject {
//        JsonObject {
//            content: Vec::new()
//        }
//    }
//}
//
//impl JsonChunk for JsonObject {
//    fn append(&mut self, node: Box<JsonChunk>) -> Option<&JsonChunk> {
//        None
//    }
//}
//
//impl Display for JsonObject {
//    fn fmt(&self, f: &mut Formatter) -> Result {
//        write!(f, "{}", "{}")
//    }
//}