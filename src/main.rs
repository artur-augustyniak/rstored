extern crate tiny_http;

#[macro_use]
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
        );


        // The type of `john` is `serde_json::Value`
        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });

        println!("first phone number: {}", john["phones"][0]);

        // Convert to a string of JSON and print it out
        println!("{}", john.to_string());


        let mut file = File::open("/proc/meminfo").unwrap(); // error 1 -> Result<File>
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap(); // error 2 -> Result<usize>


        let response = Response::from_string(john.to_string());
        request.respond(response);
    }


    //    let server = tiny_http::Server::http("0.0.0.0:0").unwrap();
    //    loop {
    //        // blocks until the next request is received
    //        let request = match server.recv() {
    //            Ok(rq) => rq,
    //            Err(e) => {
    //                println!("error: {}", e);
    //                break
    //            }
    //        };
    //
    //        // do something with the request
    //        // ...
    //    }
}
