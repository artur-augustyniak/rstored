extern crate sys_info;
extern crate libloading as lib;

use self::sys_info::*;

use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct Plugin {
    logger: Logger
}


impl Plugin {
    pub fn new(logger: Logger) -> Plugin {
        let mem = Plugin { logger: logger };
        mem.register_probe();
        mem
    }
}

fn call_dynamic() -> lib::Result<String> {
    let lib = try!(lib::Library::new("/tmp/libsomelib.so"));
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> String> = try!(lib.get(b"hosts"));
        Ok(func())
    }
}


impl Probe for Plugin {
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