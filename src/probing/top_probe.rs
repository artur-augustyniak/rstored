extern crate sys_info;


use self::sys_info::*;

use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct Top {
    logger: Logger
}


impl Top {
    pub fn new(logger: Logger) -> Top {
        let mem = Top { logger: logger };
        mem.register_probe();
        mem
    }
}

impl Probe for Top {
    fn exec(&self) -> () {
        match loadavg() {
            Ok(load) => {
                let msg = format!("@Thread: {} - load: {} {} {}, proc total: {}",
                                  self.get_thread_id(),
                                  load.one,
                                  load.five,
                                  load.fifteen,
                                  proc_total().unwrap_or(0)
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