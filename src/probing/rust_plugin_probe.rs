extern crate libloading as lib;


use super::probe::Probe;
use std::sync::Arc;
use ::logging::{Logger};
use logging::logger::syslog::Severity;
use std::io::{Error, ErrorKind};

type ExternProbe<'a> = lib::Symbol<'a, unsafe extern fn() -> String>;

#[derive(Debug)]
pub struct RustPlugin {
    logger: Logger,
    dynlib: lib::Result<lib::Library>
}


impl RustPlugin {
    pub fn new(logger: Logger) -> RustPlugin {
        let lib = lib::Library::new("/tmp/librustexampleplugin.so");
        let mem = RustPlugin { logger: logger, dynlib: lib };
        mem.register_probe();
        mem
    }
}

impl Probe for RustPlugin {
    fn register_probe(&self) -> () {
        println!("TODO custom register");
        ::probing::probe::def_register_probe(self);
    }

    fn exec(&self) -> () {
        match self.dynlib {
            Ok(ref lib) => {
                unsafe {
                    let func: ExternProbe = lib.get(b"run_probe").unwrap();
                    let json_str = func();
                    let msg = format!("@Thread: {} - json_string: {}",
                                      self.get_thread_id(),
                                      json_str
                    );
                    self.logger.log(Severity::LOG_INFO, &msg);
                }
            }
            Err(ref err) => {
                let msg = format!("{:?}", err);
                self.logger.log(Severity::LOG_ERR, &msg);
            }
        }
    }

    fn get_logger(&self) -> &Logger {
        &self.logger
    }
}

impl Drop for RustPlugin {
    fn drop(&mut self) {

        let msg = format!("RustPlugin drop, <free({:?}>)", self);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}