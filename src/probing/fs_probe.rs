extern crate sys_info;


use self::sys_info::*;

use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct Fs {
    logger: Logger
}


impl Fs {
    pub fn new(logger: Logger) -> Fs {
        let mem = Fs { logger: logger };
        mem.register_probe();
        mem
    }
}

impl Probe for Fs {
    fn exec(&self) -> () {
        match disk_info() {
            Ok(disk) => {
                let msg = format!("@Thread: {} - disk: total {} KB, free {} KB",
                                  self.get_thread_id(),
                                  disk.total,
                                  disk.free,
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