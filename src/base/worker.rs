//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

extern crate syslog;

use std::process::{exit};
use base::{Operation};
use std::thread::{spawn, sleep};

use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};


#[derive(Debug)]
pub struct Worker {
    rx: Arc<Mutex<Receiver<()>>>,
    tx: Arc<Mutex<Sender<()>>>
}

impl Worker {
    pub fn new() -> Worker {
        let (tx, rx) = mpsc::channel();
        Worker { rx: Arc::new(Mutex::new(rx)), tx: Arc::new(Mutex::new(tx)) }
    }

    pub fn start(&self) -> () {
        let rx = self.rx.clone();
        spawn(move || {
            loop {
                match rx.lock().unwrap().try_recv() {
                    Err(TryRecvError::Disconnected) => {
                        println!("Terminating, channel disconnected");
                        exit(::SIGNALING_ERROR_EXIT_CODE);
                    }
                    Ok(_) => {
                        println!("Finishing, poison pill received");
                        break
                    }
                    Err(TryRecvError::Empty) => {
                        println!("Work");
                    }
                }
                println!("or here Work");
                sleep(Duration::from_millis(2000));
            }
        });
    }
}


impl Drop for Worker {
    fn drop(&mut self) {
        self.tx.lock().unwrap().send(());
        println!("Worker Drop");
    }
}