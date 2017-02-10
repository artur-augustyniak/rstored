#[macro_use]
extern crate serde_json;
mod json;

use json::{JsonArray, JsonObject, JsonChunk};
//https://github.com/serde-rs/json
fn main() {
    let j_array = JsonArray::new();
    println!("JsonArray {} {}", j_array, j_array.area());
    let j_object = JsonObject::new();
    println!("JsonObject {} {}", j_object, j_object.area());
}