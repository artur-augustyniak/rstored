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
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use ::logging::{Logger};


#[derive(Debug)]
pub struct Worker {
    logger: Logger,
    ops: Arc<Vec<Box<Operation>>>,
    rx: Arc<Mutex<Receiver<()>>>,
    tx: Arc<Mutex<Sender<()>>>
}

impl Worker {
    pub fn new(
        logger: Logger,
        operations: Arc<Vec<Box<Operation>>>
    ) -> Worker {
        let (tx, rx) = mpsc::channel();
        Worker {
            logger: logger,
            ops: operations,
            rx: Arc::new(Mutex::new(rx)),
            tx: Arc::new(Mutex::new(tx))
        }
    }

    pub fn start(&self) -> () {
        let rx = self.rx.clone();
        let ops = self.ops.clone();
        let logger = self.logger.clone();
        spawn(move || {
            loop {
                match rx.lock().unwrap().try_recv() {
                    Err(TryRecvError::Disconnected) => {
                        logger.log("Terminating, worker channel disconnected");
                        exit(::SIGNALING_ERROR_EXIT_CODE);
                    }
                    Ok(_) => {
                        logger.log("Finishing, poison pill received");
                        break
                    }
                    Err(TryRecvError::Empty) => {
                        for op in ops.iter() {
                            op.exec();
                        }
                        sleep(Duration::from_millis(2000));
                    }
                }
            }
        });
    }
}


impl Drop for Worker {
    fn drop(&mut self) {
        self.tx.lock().unwrap().send(());
        let msg = format!("Worker drop, <free({:?}>)", self);
        self.logger.log(&msg);
    }
}