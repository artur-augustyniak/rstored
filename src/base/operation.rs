extern crate libc;
extern crate syslog;

use ::REAL_SYSLOG as REAL_SYSLOG;
use syslog::{Facility, Severity};
use std::process::Command;

fn get_thread_id() -> libc::pthread_t {
    unsafe { libc::pthread_self() }
}

pub trait Operation: Send + Sync {
    fn exec(&self) -> ();
}

#[derive(Debug)]
pub struct DebugPrint;

impl Operation for DebugPrint {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_DEBUG, &msg);
    }
}

#[derive(Debug)]
pub struct Ls;

impl Operation for Ls {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_DEBUG, &msg);
        let status = Command::new("ls").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        let msg = format!("process exited with status: {}", status);
        log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_DEBUG, &msg);
    }
}


#[derive(Debug)]
pub struct FakeSpinner;


impl Operation for FakeSpinner {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_DEBUG, &msg);
    }
}
