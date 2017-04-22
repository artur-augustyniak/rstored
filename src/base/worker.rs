//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

extern crate syslog;

use ::probing::Probe;
use std::thread::{spawn, sleep};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::sync::{Arc};
use ::logging::{Logger};
use ::base::{Config};
use logging::logger::syslog::Severity as Severity;
static CPU_ANTI_HOG_MILLIS_OFFSET: u64 = 100;

#[derive(Debug)]
pub struct Worker {
    logger: Logger,
    ops: Arc<Vec<Box<Probe>>>,
    config: Config,
    should_stop: Arc<AtomicBool>,
}

impl Worker {
    pub fn new(
        logger: Logger,
        operations: Arc<Vec<Box<Probe>>>,
        c: Config
    ) -> Worker {
        Worker {
            logger: logger,
            ops: operations,
            config: c,
            should_stop: Arc::new(AtomicBool::new(false))
        }
    }


    pub fn stop(&self) -> () {
        self.should_stop.store(true, Ordering::SeqCst);
    }

    pub fn start(&self) -> () {
        let a_len = self.ops.len();
        for i in 0..a_len {
            let ops = self.ops.clone();
            let logger = self.logger.clone();
            let timeout = self.config.get_timeout() + CPU_ANTI_HOG_MILLIS_OFFSET;
            let stop_bool = self.should_stop.clone();
            spawn(move || {
                loop {
                    if stop_bool.load(Ordering::SeqCst) {
                        let msg = format!("@Thread: {} - terminating", ops[i].get_thread_id());
                        logger.log(Severity::LOG_INFO, &msg);
                        break
                    }
                    ops[i].exec();
                    sleep(Duration::from_millis(timeout));
                }
            });
        }
    }
}


impl Drop for Worker {
    fn drop(&mut self) {

        let msg = format!("Worker drop, <free({:?}>)", self);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}