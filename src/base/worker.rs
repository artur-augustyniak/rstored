//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

extern crate syslog;

use std::process::{exit};
use ::probing::Probe;
use std::thread::{spawn, sleep};

use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use ::logging::{Logger};
use ::base::{Config};

pub use logging::logger::syslog::Severity as Severity;

static CONCURRENCY_ERROR_EXIT_CODE: i32 = 0x3;
static CPU_ANTI_HOG_MILLIS_OFFSET: u64 = 100;

#[derive(Debug)]
pub struct Worker {
    logger: Logger,
    ops: Arc<Vec<Box<Probe>>>,
    config: Config,
    rx: Arc<Mutex<Receiver<()>>>,
    tx: Arc<Mutex<Sender<()>>>
}

impl Worker {
    pub fn new(
        logger: Logger,
        operations: Arc<Vec<Box<Probe>>>,
        c: Config
    ) -> Worker {
        let (tx, rx) = mpsc::channel();
        Worker {
            logger: logger,
            ops: operations,
            config: c,
            rx: Arc::new(Mutex::new(rx)),
            tx: Arc::new(Mutex::new(tx))
        }
    }

    pub fn start(&self) -> () {
        let rx = self.rx.clone();
        let ops = self.ops.clone();
        let logger = self.logger.clone();
        let timeout = self.config.get_timeout() + CPU_ANTI_HOG_MILLIS_OFFSET;
        spawn(move || {
            loop {
                match rx.lock() {
                    Ok(guard) => {
                        match guard.try_recv() {
                            Err(TryRecvError::Disconnected) => {
                                logger.log(Severity::LOG_CRIT, "Terminating, worker channel disconnected");
                                exit(::SIGNALING_ERROR_EXIT_CODE);
                            }
                            Ok(_) => {
                                logger.log(Severity::LOG_NOTICE, "Finishing, poison pill received");
                                break
                            }
                            Err(TryRecvError::Empty) => {
                                for op in ops.iter() {
                                    op.exec();
                                }
                                sleep(Duration::from_millis(timeout));
                            }
                        }
                    }
                    Err(err) => {
                        let msg = format!("Mutex lock error: {:?}", err);
                        logger.log(Severity::LOG_ALERT, &msg);
                        exit(CONCURRENCY_ERROR_EXIT_CODE);
                    }
                }
            }
        });
    }
}


impl Drop for Worker {
    fn drop(&mut self) {

        match self.tx.lock() {
            Ok(guard) => {
                let _ = guard.send(());
                let msg = format!("Worker drop, <free({:?}>)", self);
                self.logger.log(Severity::LOG_INFO, &msg);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                exit(CONCURRENCY_ERROR_EXIT_CODE);
            }
        }
    }
}