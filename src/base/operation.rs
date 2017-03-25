extern crate libc;


use std::process::Command;

fn get_thread_id() -> libc::pthread_t {
    unsafe { libc::pthread_self() }
}

pub trait Operation: Send + Sync {
    fn exec(&self) -> ();
}

#[derive(Debug)]
pub struct DebugPrint {
    logger: ::Logger
}


impl DebugPrint {
    pub fn new(logger: ::Logger) -> DebugPrint {
        DebugPrint { logger: logger }
    }
}

impl Operation for DebugPrint {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(&msg);
    }
}

#[derive(Debug)]
pub struct Ls {
    logger: ::Logger
}

impl Ls {
    pub fn new(logger: ::Logger) -> Ls {
        Ls { logger: logger }
    }
}

impl Operation for Ls {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(&msg);
        let status = Command::new("ls").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        let msg = format!("process exited with status: {}", status);
        self.logger.log(&msg);
    }
}


#[derive(Debug)]
pub struct FakeSpinner {
    logger: ::Logger
}

impl FakeSpinner {
    pub fn new(logger: ::Logger) -> FakeSpinner {
        FakeSpinner { logger: logger }
    }
}


impl Operation for FakeSpinner {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(&msg);
    }
}
