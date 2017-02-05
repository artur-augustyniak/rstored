extern crate tiny_http;

#[macro_use]
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use tiny_http::{Server, Response};

fn main_off() {
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

extern crate ansi_term;
extern crate rand;

use self::rand::Rng;
use std::process;
use std::io::{self, BufRead};
use std::thread;
//use std::sync::mpsc;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;
use ansi_term::Colour::Red;
use std::error::Error;

static NTHREADS: i32 = 10;
static PROBE_PANIC_CODE: i32 = 127;

fn maybe_panic() -> Result<i32, Box<Error>> {

    Ok(23)
}

fn main() {
    //    let (tx, rx) = mpsc::channel();

    //    let watchdog = thread::spawn(move || {
    //        loop {
    //            //            thread::sleep(Duration::from_millis(500));
    //            match rx.try_recv() {
    //                Ok(_) | Err(TryRecvError::Disconnected) => {
    //                    println!("Terminating. ");
    //                    //                    process::exit(PROBE_PANIC_CODE);
    //                }
    //                Err(TryRecvError::Empty) => {}
    //            }
    //        }
    //    });


    let mut children = vec![];
    for i in 0..NTHREADS {
        //let tx = tx.clone();
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
            thread::sleep(Duration::from_millis(1500));
            let res:Result<i32, Box<Error>> = try!(maybe_panic()).as_result();
            Ok(())
            //            tx.send(thread_status);
            //            Ok(0)
        }));
    }
    for child in children {
        let _ = child.join();
    }

    //    let wachdog_result = watchdog.join();
    //    println!("Watchdog result {:?} {}", wachdog_result, Red.paint("All Joined after panic"));
}

//use std::thread;
//use std::time::Duration;
//use std::sync::mpsc::{self, TryRecvError};
//use std::io::{self, BufRead};
//
//fn main() {
//    println!("Press enter to terminate the child thread");
//    let (tx, rx) = mpsc::channel();
//    thread::spawn(move || {
//        loop {
//            println!("Working...");
//            thread::sleep(Duration::from_millis(500));>
//            match rx.try_recv() {
//                Ok(_) | Err(TryRecvError::Disconnected) => {
//                    println!("Terminating.");
//                    break;
//                }
//                Err(TryRecvError::Empty) => {}
//            }
//        }
//    });
//
//    let mut line = String::new();
//    let stdin = io::stdin();
//    let _ = stdin.lock().read_line(&mut line);
//
//    let _ = tx.send(());
//}
//
////#![feature(catch_panic)]
