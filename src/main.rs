#[macro_use]
extern crate serde_json;
extern crate ansi_term;
extern crate tiny_http;
//https://github.com/FillZpp/sys-info-rs/blob/master/lib.rs
extern crate sys_info;

use std::process;
use std::thread;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::time::Duration;
use sys_info::*;
use ansi_term::Colour::Red;
use tiny_http::{Server, Response};


static THREAD_ERROR_CODE: i32 = 0x1;
static NUM_THREADS: u32 = 3;
static PROBE_SLEEP_MILLIS: u64 = 100;

struct PoisonPill;

impl Drop for PoisonPill {
    fn drop(&mut self) {
        if thread::panicking() {
            process::exit(THREAD_ERROR_CODE);
        }
    }
}

fn server() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
        );

        let mut file = File::open("/proc/meminfo").unwrap(); // error 1 -> Result<File>
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap(); // error 2 -> Result<usize>


        let response = Response::from_string(contents);
        request.respond(response);
    }
}


fn main() {
    println!("{:?}", mem_info().unwrap().free);
    thread::spawn(move || {
        server();
    });

    let mut thread_handles = vec![];

    for i in 0..NUM_THREADS {
        thread_handles.push(thread::spawn(move || {
            let _b = PoisonPill;
            thread::sleep(Duration::from_millis(PROBE_SLEEP_MILLIS));
            let john = json!(
                {
                    "name": "John Doe",
                    "age": i,
                    "phones": [
                        "+44 1234567",
                        "+44 2345678"
                    ]
                }
            );

            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle.write(b"hello world {}\n");
            println!("this is thread number {}", i);
            println!("first phone number: {}", john["phones"][0]);
            println!("{}", john.to_string());
            //panic!();
        }));
    }

    for i in  0..60{
        thread::sleep(Duration::from_millis(1000));
    }

    for handle in thread_handles {
        let _ = handle.join();
    }
    println!("Some ASCII colors {}", Red.paint("All Joined"));
}
