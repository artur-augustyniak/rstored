extern crate libloading as lib;


use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;
use std::ffi::OsStr;
use std::ffi::CStr;
use std::os::raw::c_char;

type ExternProbe<'a> = lib::Symbol<'a, unsafe extern fn() -> *const c_char>;

#[derive(Debug)]
pub struct PluginProbe {
    logger: Logger,
    dynlib: lib::Result<lib::Library>
}


impl PluginProbe {
    pub fn new<P: AsRef<OsStr>>(logger: Logger, dso: P) -> PluginProbe {
        let lib = lib::Library::new(dso);
        let probe = PluginProbe { logger: logger, dynlib: lib };
        probe.register_probe();
        probe
    }
}

impl Probe for PluginProbe {
    fn register_probe(&self) -> () {
        ::probing::probe::def_register_probe(self);
    }

    fn exec(&self) -> () {
        match self.dynlib {
            Ok(ref lib) => {
                unsafe {
                    let func: ExternProbe = lib.get(b"run_probe").unwrap();
                    let extern_str = CStr::from_ptr(func());
                    let msg = format!("@Thread: {} - content: {:?}",
                                      self.get_thread_id(),
                                      extern_str
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

impl Drop for PluginProbe {
    fn drop(&mut self) {
        let msg = format!("Plugin drop, <free({:?}>)", self);
        self.logger.log(Severity::LOG_INFO, &msg);
    }
}