extern crate libloading as lib;


use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct RustPlugin {
    logger: Logger
}


impl RustPlugin {
    pub fn new(logger: Logger) -> RustPlugin {
        let mem = RustPlugin { logger: logger };
        mem.register_probe();
        mem
    }
}

fn call_dynamic() -> lib::Result<String> {
    let lib = try!(lib::Library::new("/tmp/librustexampleplugin.so"));
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> String> = try!(lib.get(b"run_probe"));
        Ok(func())
    }
}


impl Probe for RustPlugin {
    fn exec(&self) -> () {
        match call_dynamic() {
            Ok(json_str) => {
                let msg = format!("@Thread: {} - json_string: {}",
                                  self.get_thread_id(),
                                  json_str
                );
                self.logger.log(Severity::LOG_INFO, &msg);
            }

            Err(err) => {
                let msg = format!("{:?}", err);
                self.logger.log(Severity::LOG_ERR, &msg);
            }
        }
    }

    fn get_logger(&self) -> &Logger {
        &self.logger
    }
}