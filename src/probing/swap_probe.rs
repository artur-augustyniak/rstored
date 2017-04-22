extern crate sys_info;


use self::sys_info::*;

use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct Swap {
    logger: Logger
}


impl Swap {
    pub fn new(logger: Logger) -> Swap {
        let swap = Swap { logger: logger };
        swap.register_probe();
        swap
    }
}

impl Probe for Swap {
    fn exec(&self) -> () {
        match mem_info() {
            Ok(swap) => {
                let msg = format!("@Thread: {} - swap: total {} KB, free {} KB",
                                  self.get_thread_id(),
                                  swap.swap_total,
                                  swap.swap_free
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




