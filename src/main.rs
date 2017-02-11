#[macro_use]
extern crate serde_json;

mod json;

use json::{JsonArray, JsonObject, JsonChunk, JsonBuilder};


fn manual() {
    let mut root_array = JsonArray::new();
    let inner_array1 = JsonArray::new();

    let mut inner_array2 = JsonArray::new();
    let mut inner_array3 = JsonArray::new();

    let inner_array4 = JsonArray::new();

    inner_array2.append(Box::new(inner_array3));
    inner_array2.append(Box::new(inner_array4));
    root_array.append(Box::new(inner_array1));
    root_array.append(Box::new(inner_array2));

    println!("{}", root_array);


    let mut new_root_array = JsonArray::new();
    new_root_array.append(Box::new(root_array));
    println!("{}", new_root_array);


    //    let mut j_array = JsonArray::new();
    //
    //
    //
    //
    ////    let  j_object = JsonObject::new();
    ////    j_array.append(Box::new(j_object));
    //    let mut j_array2 = JsonArray::new();
    //    let j_array3 = JsonArray::new();
    //    let j_array4 = JsonArray::new();
    //
    //    j_array2.append(Box::new(j_array4));
    //    j_array.append(Box::new(j_array3));
    //
    //
    ////
    ////    let mut combined = j_array.append(Box::new(j_array2));
    ////    match combined {
    ////        Some(x) => println!("{}", x),
    ////        None => println!("None error"),
    ////    }
    //
}

fn main() {
//    manual();
    let  j_array = JsonArray::new();
    let jba = JsonBuilder::new(j_array);
    jba.print();


}
