#[macro_use]
extern crate serde_json;

mod json;

use json::{JsonArray, JsonObject, JsonChunk};

fn main() {
    let mut j_array = JsonArray::new();
    let  j_object = JsonObject::new();
    j_array.append(Box::new(j_object));
    let mut j_array2 = JsonArray::new();
    let j_array3 = JsonArray::new();
    let j_array4 = JsonArray::new();

    j_array2.append(Box::new(j_array4));
    j_array.append(Box::new(j_array3));



    let mut combined = j_array.append(Box::new(j_array2));
    match combined {
        Some(x) => println!("{}", x),
        None => println!("None error"),
    }


}