extern crate libc;

use std::fmt::Debug;
use std::process::Command;
use ::logging::{Logger};
use logging::logger::syslog::Severity;

fn get_thread_id() -> libc::pthread_t {
    unsafe { libc::pthread_self() }
}


pub trait Operation: Send + Sync + Debug {
    fn exec(&self) -> ();
}

#[derive(Debug)]
pub struct DebugPrint {
    logger: Logger
}


impl DebugPrint {
    pub fn new(logger: Logger) -> DebugPrint {
        DebugPrint { logger: logger }
    }
}

impl Operation for DebugPrint {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}

#[derive(Debug)]
pub struct Ls {
    logger: Logger
}

impl Ls {
    pub fn new(logger: Logger) -> Ls {
        Ls { logger: logger }
    }
}

impl Operation for Ls {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(Severity::LOG_INFO,&msg);
        let status = Command::new("ls").status().unwrap_or_else(|e| {
            let msg = format!("failed to execute process: {}", e);
            self.logger.log(Severity::LOG_ERR, &msg);
            panic!(msg)
        });
        let msg = format!("process exited with status: {}", status);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}


#[derive(Debug)]
pub struct FakeSpinner {
    logger: Logger
}

impl FakeSpinner {
    pub fn new(logger: Logger) -> FakeSpinner {
        FakeSpinner { logger: logger }
    }
}


impl Operation for FakeSpinner {
    fn exec(&self) -> () {
        let msg = format!("Thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}



extern crate sys_info;

use self::sys_info::*;


#[derive(Debug)]
pub struct FreeMem {
    logger: Logger
}


impl FreeMem {
    pub fn new(logger: Logger) -> FreeMem {
        FreeMem { logger: logger }
    }
}

impl Operation for FreeMem {
    fn exec(&self) -> () {
        println!("os: {} {}", os_type().unwrap(), os_release().unwrap());
        println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());
        println!("proc total: {}", proc_total().unwrap());
        let load = loadavg().unwrap();
        println!("load: {} {} {}", load.one, load.five, load.fifteen);
        let mem = mem_info().unwrap();
        println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
                 mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
        println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
        let disk = disk_info().unwrap();
        println!("disk: total {} KB, free {} KB", disk.total, disk.free);
        println!("hostname: {}", hostname().unwrap());

        let msg = format!("FreeMem thread id {:?} {:?} working... ", get_thread_id(), self);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}