extern crate libc;

use std::fmt::Debug;
use ::logging::{Logger};
use logging::logger::syslog::Severity;

pub trait Probe: Send + Sync + Debug {
    fn exec(&self) -> ();

    fn get_logger(&self) -> &Logger;

    fn get_thread_id(&self) -> libc::pthread_t {
        get_thread_id()
    }

    fn register_probe(&self) -> () {
        def_register_probe(self);
    }
}

pub fn def_register_probe<T: Probe + ? Sized>(x: &T) {
    let msg = format!("@Thread: {} - Registering probe: {:?} ", x.get_thread_id(), x);
    x.get_logger().log(Severity::LOG_INFO, &msg);
}

fn get_thread_id() -> libc::pthread_t {
    unsafe { libc::pthread_self() }
}



