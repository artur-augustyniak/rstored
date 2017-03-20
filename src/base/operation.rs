extern crate libc;

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
        println!("[+] Thread id {:?} {:?} working... ", get_thread_id(), self);
    }
}

#[derive(Debug)]
pub struct Ls;

impl Operation for Ls {
    fn exec(&self) -> () {
        println!("[+] Thread id {:?} {:?} working... ", get_thread_id(), self);
        let status = Command::new("ls").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        println!("[+] process exited with status: {}", status);
    }
}


#[derive(Debug)]
pub struct FakeSpinner;


impl Operation for FakeSpinner {
    fn exec(&self) -> () {
        println!("[+] Thread id {:?} {:?} working... ", get_thread_id(), self);
    }
}
